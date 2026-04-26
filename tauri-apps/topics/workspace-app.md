# Scenario: Workspace App

Use this profile for productivity apps where users create, organize, and edit owned content with moderate persistence.

Examples: notes app, snippet manager, bookmark organizer.

## Architecture Defaults

1. Keep content domain models explicit and versionable.
2. Use local persistence (SQLite/file-backed) with repository-style backend modules.
3. Keep command wrappers in `src/lib/tauri.ts` and shared frontend model types in `src/lib/types.ts`.
4. Treat import/export as product features, not one-off utilities.

## UI/UX Focus

1. Multi-pane layouts for navigation + editor + metadata.
2. Draft/save/dirty-state behavior that avoids data loss.
3. Search/filter/tag flows optimized for large personal collections.
4. Undo-friendly interactions for destructive operations.

## Command Guidance

1. Keep CRUD commands predictable and strongly typed.
2. Batch operations should report partial failures clearly.
3. Use background tasks for heavy import/export/reindex operations.
4. Keep autosave and conflict handling explicit.

## Data Model Guidance

1. Define content, metadata, and index/search models separately.
2. Include stable IDs and timestamps for sync/export potential.
3. Version persisted records to support migrations.
4. Keep validation rules close to model definitions.

## Common Overlaps

1. With data app when users inspect structured datasets.
2. With job app when background indexing/sync becomes central.
3. With control panel app when workspace config drives external systems.
