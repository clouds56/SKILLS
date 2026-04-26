# Scenario: Tooling App

Use this profile for desktop wrappers around command-line tools or backend actions where files, progress, logs, and task control matter.

Examples: ffmpeg converter, aria2c GUI, image compression tool.

## Architecture Defaults

1. Isolate command orchestration in Rust command modules.
2. Use event streams for progress/log updates (`task://` or `job://` namespaces).
3. Keep invocation wrappers in `src/lib/tauri.ts` and payload/result contracts in `src/lib/types.ts`.
4. Keep `App.tsx` focused on composition; place workflow logic in page-level hooks/services.

## UI/UX Focus

1. Explicit input/output file selection and validation.
2. Visible run state: queued, running, completed, failed, cancelled.
3. Log panel with copy/export support.
4. Retry and cancel controls near task status.

## Command Guidance

1. Commands should be async and cancellable.
2. Emit structured progress events with stable payload shape.
3. Include clear error categories (user input, tool missing, runtime failure).
4. Keep process spawning and cleanup centralized in backend utilities.

## Data Model Guidance

1. Define `TaskRun`, `TaskStatus`, and `TaskProgress` types in `src/lib/types.ts`.
2. If persistence is needed, promote to durable `JobRecord` model and DB-backed history.
3. Keep log/event payloads versionable to avoid UI breakage.

## Common Overlaps

1. With job app when runs must survive restarts.
2. With workspace app when presets/projects are persisted heavily.
3. With control panel app when tooling manages remote services.
