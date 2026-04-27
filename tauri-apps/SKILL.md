---
name: tauri-apps
description: 'Create and refactor maintainable Tauri 2.0 apps with pnpm-first setup, required icon generation, async command patterns, events for long-running work, and modular React/Rust architecture.'
argument-hint: 'Describe the app tabs, data sources, and persistence needs.'
user-invocable: true
disable-model-invocation: false
---

# Tauri Apps Skill Index

Use this skill when creating or refactoring Tauri 2.0 apps that should stay modular and production-friendly.

## Requirements Snapshot
1. Use Tauri 2.0.
2. Prefer pnpm.
3. Add `@tauri-apps/cli` to `devDependencies`.
4. Do not write `packageManager` into `package.json`.
5. Ensure icons are ready before first run/build.
6. Use async command handlers for network and file I/O.
7. Keep `App.tsx` and `lib.rs` slim; push domain logic into modules.
8. Keep command invocation wrappers in `src/lib/tauri.ts`.
9. Keep shared command/model types in `src/lib/types.ts`.
10. Treat this as a maintainable production app, not a prototype.

## Feature Planning Gate (Required)

Before implementing any new feature, confirm a brief plan that covers all three areas:
1. Screen plan (UI/UX): target screen(s), interaction flow, and loading/error/empty states.
2. Command plan: new/changed Tauri commands, async behavior, and event usage for long-running work.
3. Data model plan: TypeScript types in `src/lib/types.ts`, Rust structs/enums if needed, and mapping between them.

Do not start coding until this plan is explicitly confirmed.

## Documents
- Setup and bootstrap checklist: [create.md](./create.md)
- Scenario guide: [topics/simple-ui.md](./topics/simple-ui.md)
- Scenario guide: [topics/tooling-app.md](./topics/tooling-app.md)
- Scenario guide: [topics/data-app.md](./topics/data-app.md)
- Scenario guide: [topics/job-app.md](./topics/job-app.md)
- Scenario guide: [topics/workspace-app.md](./topics/workspace-app.md)
- Scenario guide: [topics/control-panel.md](./topics/control-panel.md)
- Scenario guide: [topics/floating-window.md](./topics/floating-window.md)
- Scenario overlap guide: [topics/scenario-overlap.md](./topics/scenario-overlap.md)
- Scenario guide: [topics/tasks.md](./topics/tasks.md)

## Recommended Project Shape

### Frontend
```text
src/
  App.tsx
  pages/
    HomePage.tsx
    SettingsPage.tsx
    ...one page per tab...
  components/
    ...reusable UI components...
  lib/
    tauri.ts
    types.ts
    state.ts
    ...shared helpers...
```

### Rust (Tauri)
```text
src-tauri/src/
  main.rs
  lib.rs
  commands/
    mod.rs
    ...command groups by domain...
  utils/
    ...shared utility modules...
  db/
    ...persistence and repository code...
```

## Example Prompts
- "Create a Tauri 2 app with three tabs using this skill docs structure."
- "Refactor my Tauri app so `App.tsx` and `lib.rs` are thin and event-driven for long tasks."
- "Apply the control panel topic guide and add progress events for background jobs."
