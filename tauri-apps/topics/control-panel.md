# Scenario: Control Panel App

Use this profile for apps that configure, monitor, and operate local or remote systems through structured actions and status views.

Examples: Docker GUI, server dashboard, service manager.

## Architecture Defaults

1. Separate command/control plane from telemetry/read plane.
2. Keep backend adapters modular by provider/system type.
3. Keep command wrappers in `src/lib/tauri.ts` with explicit auth/session handling.
4. Keep shared state and response types in `src/lib/types.ts`.

## UI/UX Focus

1. Status-first dashboard with actionable alerts.
2. Safe action design: confirmation for risky operations and clear scope labels.
3. Operator workflows with filters by environment, host, or service.
4. Freshness indicators for polling/streaming data.

## Command Guidance

1. Distinguish read-only commands from mutating actions.
2. Normalize errors across heterogeneous backends.
3. Support polling and event-driven updates where possible.
4. Add cancellation/timeouts for remote operations.

## Data Model Guidance

1. Model resources, actions, and health/status separately.
2. Include provenance fields (source, environment, last update).
3. Keep action request/response payloads auditable.
4. Encode permission/capability hints to drive UI affordances.

## Common Overlaps

1. With tooling app when control actions wrap local CLIs.
2. With job app when operations are long-running and recoverable.
3. With data app when monitoring and analytics are both first-class.
