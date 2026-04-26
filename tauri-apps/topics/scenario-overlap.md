# Scenario Overlap Guide

Scenarios are composable. Pick a primary scenario, then layer secondary requirements deliberately.

## How To Combine

1. Choose one primary scenario based on the app's core user value.
2. Add secondary scenario rules only where requirements clearly demand them.
3. Confirm a single feature plan covering UI/UX, commands, and data model before implementation.
4. Keep shared contracts centralized in `src/lib/types.ts` and command wrappers in `src/lib/tauri.ts`.

## Common Combinations

1. Tooling + Job: command wrappers with durable run history and restart recovery.
2. Data + Job: heavy scans/analysis as background jobs with streamed partial results.
3. Workspace + Data: persistent user content plus rich query/filter/visualization.
4. Control Panel + Tooling: operational dashboard that also invokes CLI-backed actions.
5. Workspace + Job: user-defined pipelines or scheduled processing on saved content.

## Escalation Signals

Move to a richer scenario when you add:
1. Durable run tracking/history -> job rules.
2. Significant content persistence/migrations -> workspace rules.
3. Large structured datasets with exploration workflows -> data rules.
4. Remote system operations and health views -> control panel rules.

## Scope Guardrails

1. Do not introduce job persistence for simple one-shot interactions.
2. Do not over-model data entities before usage patterns are known.
3. Keep event names and payloads stable once published to UI.
4. Revisit scenario choice when feature complexity changes materially.
