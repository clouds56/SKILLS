# Scenario: Data App

Use this profile for apps centered on reading structured data and helping users inspect, filter, and visualize it.

Examples: system monitor, ncdu-style disk usage GUI, CSV/JSON analytics viewer.

## Architecture Defaults

1. Separate ingestion, transform, and presentation layers.
2. Use Rust for heavy parsing/scanning where performance matters.
3. Keep typed query/filter contracts in `src/lib/types.ts`.
4. Keep command wrappers in `src/lib/tauri.ts` with narrow interfaces.

## UI/UX Focus

1. Fast filter/sort interactions with debounced controls.
2. Table/list + detail-pane patterns for exploration.
3. Empty/no-match/error states that explain next actions.
4. Visual summaries that stay synchronized with filter context.

## Command Guidance

1. Prefer paginated or chunked responses for large datasets.
2. Stream progress and partial results for expensive scans.
3. Keep command payloads explicit about units and ranges.
4. Avoid returning unbounded arrays for large sources.

## Data Model Guidance

1. Define separate types for raw source rows vs. view models.
2. Track filter state as a serializable object.
3. Add schema versioning for imported/exported artifacts.
4. Use stable IDs for row selection and drill-down continuity.

## Common Overlaps

1. With tooling app when data comes from external CLI pipelines.
2. With job app when collection/analysis runs are long-lived.
3. With workspace app when users save reports, dashboards, or views.
