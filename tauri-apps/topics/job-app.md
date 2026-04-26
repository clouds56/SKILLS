# Scenario: Job App

Use this profile for apps built around spawning, tracking, and recovering long-running background workflows.

Examples: ML training manager, SSH batch processing runner.

## Architecture Defaults

1. Treat jobs as first-class domain entities with lifecycle state.
2. Persist job metadata/history in SQLite (or equivalent) for restart recovery.
3. Keep job orchestration in backend modules; keep UI as control/observability surface.
4. Keep frontend contracts in `src/lib/types.ts` and backend invocation in `src/lib/tauri.ts`.

## UI/UX Focus

1. Queue/runs/history views with clear status badges.
2. Per-job details: logs, progress timeline, artifacts, and error reason.
3. Recover/reconcile messaging after app restart.
4. Safe controls for stop/retry/resume where supported.

## Command Guidance

1. Commands should support start/stop/status/list semantics.
2. Emit status/progress/events/log streams with stable event names.
3. Persist state before emitting terminal events.
4. Rehydrate running/non-terminal jobs on startup.

## Data Model Guidance

1. Define `JobRecord`, `JobStatus`, `JobProgress`, and `JobEvent` types.
2. Keep status transitions explicit and validated.
3. Store timestamps for created/started/updated/finished phases.
4. Track correlation IDs between frontend requests and backend job IDs.

## Common Overlaps

1. With tooling app when jobs wrap CLI processes.
2. With data app when jobs produce large analytical outputs.
3. With workspace app when job definitions/pipelines are user-editable.
