# Scenario: Simple UI App

Use this profile for self-contained desktop apps with local state only and no backend-managed long-running work.

Examples: base64 encoder/decoder, Pomodoro timer, OTP authenticator, markdown previewer.

## Architecture Defaults

1. Keep state in frontend (`useState`, `useReducer`, or a small store in `src/lib/state.ts`).
2. Minimize Rust commands; use them only for platform APIs that web code cannot access.
3. Keep `src/lib/tauri.ts` small, explicit, and typed.
4. Keep shared frontend types in `src/lib/types.ts`.

## UI/UX Focus

1. Fast interactions with immediate feedback.
2. Keyboard-first controls where relevant.
3. Clear empty/default states instead of placeholder-only screens.
4. Inline validation and short error messages.

## Command Guidance

1. Prefer direct local computation in frontend when feasible.
2. If a command exists, keep it deterministic and low latency.
3. Avoid background job machinery unless requirements change.

## Data Model Guidance

1. Model UI state explicitly in TypeScript unions/interfaces.
2. Use simple serialization boundaries when crossing Tauri command calls.
3. Avoid premature DB schemas.

## Common Triggers To Reclassify

Move toward other scenarios when you add:
1. CLI/process control and logs -> tooling app.
2. Structured datasets and query/filter flows -> data app.
3. Durable content CRUD -> workspace app.
4. Persistent background execution -> job app.
