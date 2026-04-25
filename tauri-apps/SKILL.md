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

## Documents
- Setup and bootstrap checklist: [create.md](./create.md)
- Scenario guides: [topics/control-panel.md](./topics/control-panel.md)

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
