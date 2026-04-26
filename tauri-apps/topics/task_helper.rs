use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tracing::field::{Field, Visit};
use tracing::span::Id;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;

#[derive(Clone)]
struct ReqMeta {
  req_id: u64,
  req_type: String,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
  Started,
  InProgress,
  Completed,
  Failed,
  Cancelled,
  // Exited is an automatic lifecycle fallback emitted when the span ends.
  // It should not be treated as replacing terminal statuses like completed/failed.
  Exited,
}

#[derive(Default)]
struct SpanFieldVisitor {
  req_id: Option<u64>,
  req_type: Option<String>,
}

impl Visit for SpanFieldVisitor {
  fn record_u64(&mut self, field: &Field, value: u64) {
    if field.name() == "req_id" {
      self.req_id = Some(value);
    }
  }

  fn record_str(&mut self, field: &Field, value: &str) {
    if field.name() == "req_type" {
      self.req_type = Some(value.to_string());
    }
  }

  fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {}
}

#[derive(Clone)]
pub struct TaskEmitter {
  app: AppHandle,
  req_id: u64,
  req_type: String,
}

impl TaskEmitter {
  pub fn progress(&self, current: i64, total: Option<i64>, msg: Option<&str>) {
    let payload = serde_json::json!({
      "req_id": self.req_id,
      "req_type": self.req_type,
      "current": current,
      "total": total,
      "message": msg,
    });
    let _ = self.app.emit(format!("task://{}/progress", self.req_type), payload);
  }

  pub fn status(&self, status: TaskStatus) {
    let payload = serde_json::json!({
      "req_id": self.req_id,
      "req_type": self.req_type,
      "status": status,
    });
    let _ = self.app.emit(format!("task://{}/status", self.req_type), payload);
  }

  pub fn event<T: Serialize>(&self, event: &T) {
    let payload = serde_json::json!({
      "req_id": self.req_id,
      "req_type": self.req_type,
      "event": event,
    });
    let _ = self.app.emit(format!("task://{}/events", self.req_type), payload);
  }
}

pub trait AppEmitExt {
  fn task(&self, req_id: u64, req_type: &str) -> TaskEmitter;
  fn progress(&self, req_id: u64, req_type: &str, current: i64, total: Option<i64>, msg: Option<&str>);
  fn status(&self, req_id: u64, req_type: &str, status: TaskStatus);
  fn event<T: Serialize>(&self, req_id: u64, req_type: &str, event: &T);
}

impl AppEmitExt for AppHandle {
  fn task(&self, req_id: u64, req_type: &str) -> TaskEmitter {
    TaskEmitter {
      app: self.clone(),
      req_id,
      req_type: req_type.to_string(),
    }
  }

  fn progress(&self, req_id: u64, req_type: &str, current: i64, total: Option<i64>, msg: Option<&str>) {
    self.task(req_id, req_type).progress(current, total, msg)
  }

  fn status(&self, req_id: u64, req_type: &str, status: TaskStatus) {
    self.task(req_id, req_type).status(status)
  }

  fn event<T: Serialize>(&self, req_id: u64, req_type: &str, event: &T) {
    self.task(req_id, req_type).event(event)
  }
}

pub struct FrontendEventLayer {
  pub app: AppHandle,
}

impl<S> tracing_subscriber::Layer<S> for FrontendEventLayer
where
  S: Subscriber + for<'a> LookupSpan<'a>,
{
  fn on_new_span(&self, attrs: &tracing::span::Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
    if let Some(span_ref) = ctx.span(id) {
      let mut visitor = SpanFieldVisitor::default();
      attrs.record(&mut visitor);
      if let (Some(req_id), Some(req_type)) = (visitor.req_id, visitor.req_type) {
        span_ref.extensions_mut().insert(ReqMeta { req_id, req_type });
      }
    }
  }

  fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
    if let Some(span_ref) = ctx.span(id) {
      let ext = span_ref.extensions();
      if let Some(meta) = ext.get::<ReqMeta>() {
        self.app.status(meta.req_id, &meta.req_type, TaskStatus::Started);
      }
    }
  }

  fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
    if let Some(span_ref) = ctx.span(id) {
      let ext = span_ref.extensions();
      if let Some(meta) = ext.get::<ReqMeta>() {
        // Fallback lifecycle signal only: do not overwrite terminal status.
        self.app.status(meta.req_id, &meta.req_type, TaskStatus::Exited);
      }
    }
  }

  fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
    if let Some(scope) = ctx.event_scope(event) {
      if let Some(span_ref) = scope.from_root().last() {
        let ext = span_ref.extensions();
        if let Some(meta) = ext.get::<ReqMeta>() {
          let payload = serde_json::json!({
            "req_id": meta.req_id,
            "req_type": meta.req_type,
            "level": format!("{}", event.metadata().level()),
            "target": event.metadata().target(),
            "name": event.metadata().name(),
          });
          let _ = self.app.emit(format!("task://{}/log", meta.req_type), payload);
        }
      }
    }
  }
}

pub async fn run_task(app: AppHandle, req_id: u64) -> Result<(), String> {
  let req_type = "do_this_task";
  let task = app.task(req_id, req_type);

  let span = tracing::info_span!("task", req_id, req_type);
  let _enter = span.enter();

  task.event(&serde_json::json!({ "phase": "queued" }));
  task.progress(10, Some(100), Some("loading"));
  tracing::debug!("task lifecycle message");
  task.event(&serde_json::json!({ "phase": "done" }));

  Ok(())
}
