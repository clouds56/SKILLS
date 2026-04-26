# Scenario: Tasks (Ephemeral Task + Persistent Job)

This scenario defines a two-track execution model for long-running backend work in Tauri apps.

## Model Summary

There are two kinds of work units:

1. `task` (ephemeral): In-memory, tied to current app runtime, lost on app exit.
2. `job` (persistent): Durable, survives app restarts, with history in SQLite.

Use `task` when best-effort runtime progress is enough. Use `job` when audit/history/recovery is required.

## Identity Rules

1. Ephemeral task IDs are incremental request IDs (JSON-RPC style, for example `u64`).
2. Persistent job IDs are random IDs (for example UUIDv4 string).
3. Never reuse IDs inside the same category.

## Ephemeral Task Lifecycle

### Command Style

Long-running task commands are async and complete only when the work is done:

```rust
// Example shape only
async fn do_this_task(req_id: u64, args: DoTaskArgs) -> Result<DoTaskResult, TaskError>
```

### Control Surface

Expose consistent controls:

1. `get_task_status(req_id)`
2. `cancel_task(req_id)`

### Events and Streams

Use `task://` event namespace per action:

1. `task://do_this_task/status` (started, running, completed, failed, cancelled)
2. `task://do_this_task/events` (domain events)
3. `task://do_this_task/progress` (progress updates)
4. Optional: `task://do_this_task/log`
5. Optional: `task://do_this_task/metrics`

Use app-level helpers for emission:

```rust
use crate::task_helper::TaskStatus;

let task = app.task(req_id, "do_this_task");
task.progress(current, total, msg);
task.status(TaskStatus::Started);
task.event(event); // event is serialized to JSON
debug!("task lifecycle message");
```

`AppEmitExt` still has low-level methods, but `app.task(req_id, req_type)` creates a contextual emitter so repeated calls do not need `req_id`/`req_type`.

Detailed scaffold:

```rust
// Put the full helper implementation in task_helper.rs.
use crate::task_helper::{AppEmitExt, TaskStatus};

pub async fn run_task(app: tauri::AppHandle, req_id: u64) -> Result<(), String> {
  let req_type = "do_this_task";
  let task = app.task(req_id, req_type);

  let span = tracing::info_span!("task", req_id, req_type);
  let _enter = span.enter();

  task.status(TaskStatus::InProgress);
  task.event(&serde_json::json!({ "phase": "queued" }));
  task.progress(10, Some(100), Some("loading"));
  tracing::debug!("task lifecycle message");
  task.event(&serde_json::json!({ "phase": "done" }));
  Ok(())
}
```

Full app helper and tracing layer implementation: [task_helper.rs](./task_helper.rs)

Recommended runtime wiring:

1. Keep a custom tracing subscriber layer that forwards selected logs/events to frontend.
2. Emit via both typed events and tracing so UI and diagnostics stay aligned.
3. Keep frontend payloads JSON-serializable and stable.

### In-Memory State

Track active work in runtime memory:

```rust
use std::collections::HashMap;

type ReqId = u64;

struct TaskHandle {
  // JoinHandle, cancellation token, timestamps, and snapshots.
}

type RunningTasks = HashMap<ReqId, TaskHandle>;
```

Recommended behavior:

1. Insert into map before spawning.
2. Remove on terminal state.
3. Return `not_found` for unknown `req_id`.

## Persistent Job Lifecycle

Persistent jobs mirror task semantics but use durable state as source of truth.

### Core Trait

Define a `Job` trait using `async_trait` with quick sync controls and async status sampling.

```rust
use async_trait::async_trait;

pub struct ProgressSnapshot {
  pub current: i64,
  pub total: Option<i64>,
  pub message: Option<String>,
  pub updated_at_ms: i64,
}

pub struct JobEntry {
  pub job_id: String,
  pub kind: String,
  pub pid: Option<u32>,
  pub status: String,
  pub payload_json: String,
  pub stdout_path: Option<String>,
  pub stderr_path: Option<String>,
}

#[async_trait]
pub trait Job: Send + Sync {
  // Quick control path: request launch and return without long blocking.
  fn spawn(&mut self, app: &tauri::AppHandle) -> Result<(), JobError>;

  // Quick control path: request stop and return quickly.
  fn kill(&mut self) -> Result<(), JobError>;

  // Await terminal completion when the caller explicitly needs to wait.
  async fn join(&mut self) -> Result<(), JobError>;

  // Progress is inferred from running job state and returned.
  async fn sample_progress(&self) -> Result<ProgressSnapshot, JobError>;

  fn from_entry(entry: JobEntry) -> Result<Self, JobError>
  where
    Self: Sized;
}

pub struct JobError(pub String);
```

Notes:

1. A `pid` is commonly tracked, but alternative trackers are valid.
2. `from_entry` is required for restart recovery.
3. `spawn` and `kill` should be fast control methods.
4. `join` provides explicit wait semantics when a caller needs terminal completion.
5. `JobExit` is intentionally omitted in this simplified trait shape.

### Persistence Rules

1. SQLite is the source of truth.
2. Every state transition writes to DB first, then emits events.
3. Keep an append-only event/history table for diagnostics.
4. Jobs may optionally persist stdout/stderr locations.

### Common SQLite Schema

Use one shared schema for all persistent jobs:

```sql
create table if not exists jobs (
  job_id text primary key,
  kind text not null,
  status text not null,
  pid integer,
  payload_json text not null,
  current_progress integer,
  total_progress integer,
  progress_message text,
  progress_updated_at_ms integer,
  stdout_path text,
  stderr_path text,
  created_at_ms integer not null,
  updated_at_ms integer not null,
  started_at_ms integer,
  finished_at_ms integer
);

create table if not exists job_progress (
  id integer primary key autoincrement,
  job_id text not null,
  current integer not null,
  total integer,
  message text,
  ts_ms integer not null,
  foreign key(job_id) references jobs(job_id)
);

create table if not exists job_events (
  id integer primary key autoincrement,
  job_id text not null,
  event_type text not null,
  event_json text not null,
  ts_ms integer not null,
  foreign key(job_id) references jobs(job_id)
);

create index if not exists idx_jobs_status on jobs(status);
create index if not exists idx_job_events_job_id on job_events(job_id, ts_ms);
create index if not exists idx_job_progress_job_id on job_progress(job_id, ts_ms);
```

`job_progress` is optional. If not used, persist latest progress snapshot in `jobs.current_progress` and `jobs.total_progress` (plus message/timestamp columns).

### Startup Recovery

On app startup:

1. Scan DB for non-terminal jobs.
2. Reconstruct job instances via `Job::from_entry`.
3. Re-attach tracking (pid-based or alternative handle).
4. Emit reconciliation events so UI state converges.
5. Resume sampling progress through `sample_progress`.

### Events and Streams

Use `job://` namespace mirroring task events:

1. `job://<kind>/status`
2. `job://<kind>/events`
3. `job://<kind>/progress`
4. Optional: `job://<kind>/log`
5. Optional: `job://<kind>/metrics`

## Decision Points

Choose `task` when:

1. Work can be abandoned on app exit.
2. No history/audit requirement exists.
3. Lower complexity is preferred.

Choose `job` when:

1. Work must survive app offline/restart.
2. You need durable history and traceability.
3. Recovery/reconciliation is required.

## Quality Checks

1. IDs obey category rules: incremental for tasks, random for jobs.
2. Event names are stable and namespaced (`task://`, `job://`).
3. Terminal states always clean memory handles and persist final status.
4. Startup recovery correctly rehydrates in-flight jobs from DB.
5. Cancel/status endpoints behave consistently for unknown IDs.
6. Progress is always queryable via either `job_progress` or `jobs.current_progress`/`jobs.total_progress`.
