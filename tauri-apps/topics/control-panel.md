# Scenario: Control Panel (Events + Long-Running Tasks)

This scenario explains how to build a control panel that starts long tasks and streams status updates to the UI.

## Goal

- User clicks a control to start a long-running backend task.
- Rust backend performs work asynchronously.
- Frontend receives progress and completion updates via events.
- UI stays responsive and can offer cancel/retry behavior.

## Recommended Flow

1. Frontend invokes a Tauri command to start a job.
2. Command returns quickly with a `job_id`.
3. Backend spawns async work and emits events tagged by `job_id`.
4. Frontend listens for events and updates view state.
5. Backend emits final success/failure event.

## Event Contract

Use stable, explicit event names:

- `job://started`
- `job://progress`
- `job://completed`
- `job://failed`
- `job://cancelled`

Payload shape (example):

```json
{
  "job_id": "job_123",
  "status": "running",
  "progress": 42,
  "message": "Processed 42 of 100",
  "ts": "2026-04-25T12:00:00Z"
}
```

## Rust Guidance (Tauri 2.0)

1. Make long-task commands `async`.
2. Return immediately after scheduling the task; do not block UI-call path.
3. Use `tauri::AppHandle` to emit events to the frontend.
4. Keep shared job state in a synchronized store (for example a map keyed by `job_id`).
5. Add cancellation support (for example cancellation token per job).
6. Emit concise `tracing` logs (`info` for user actions, `debug` for internals).

## Frontend Guidance

1. Subscribe to events when page/component mounts.
2. Filter events by `job_id`.
3. Store job status in `src/lib/state.ts`.
4. Unsubscribe listeners on unmount.
5. Handle duplicate or out-of-order events defensively.

## Minimal Lifecycle Pattern

1. `start_job` command invoked.
2. Show status: `queued`.
3. Receive `job://started`.
4. Receive periodic `job://progress` events.
5. Receive one terminal event: `completed` or `failed` or `cancelled`.
6. Render summary and keep latest logs/error for support.

## Operational Notes

1. Include `job_id`, action name, and elapsed duration in logs.
2. Prefer throttled progress events (for example every 100 to 500 ms) to avoid event flood.
3. Never emit sensitive data in event payloads.
4. Keep `info` logging to about 1 to 5 lines per user action.

## Common Pitfalls

1. Blocking in command handler instead of spawning async work.
2. Using inconsistent event names across modules.
3. Forgetting listener cleanup in frontend.
4. Returning opaque errors that UI cannot present.
5. Emitting too many progress events and degrading UI performance.
