---
name: tauri-app-creation
description: 'Create maintainable Tauri 2.0 apps with pnpm-first setup, required icon generation, async Rust commands, modular React and Rust architecture, and tracing-based logging standards. Use for new Tauri app scaffolding, refactors toward cleaner structure, and implementation reviews.'
argument-hint: 'Describe the app tabs, data sources, and persistence needs.'
user-invocable: true
disable-model-invocation: false
---

# Tauri App Creation (Tauri 2.0)

## What This Skill Produces
- A Tauri 2.0 app scaffold and implementation plan.
- A modular frontend and backend structure that avoids oversized root files.
- Async-safe command patterns for network and file I/O.
- A startup-ready icon set and practical logging conventions.

## When To Use
- Creating a new desktop app with Tauri.
- Refactoring an existing Tauri app with oversized `App.tsx` or `lib.rs`.
- Reviewing project structure, command design, and operational logging.

## Standards
1. Use Tauri 2.0.
2. Prefer pnpm.
3. Add `@tauri-apps/cli` to dev dependencies.
4. Do not write `packageManager` into `package.json`.
5. Generate or provide app icons before first run/build.
6. Use async commands for network and file I/O work.
7. Keep frontend and Rust backend modular with clear folders.
8. Use `tracing` for logs; at `info` level, keep logs concise (about 1 to 5 lines per user action).

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

## Resources
- Initialization prompts: [init.md](./init.md)

## Procedure

### 1. Initialize and Pin Baseline
Use [init.md](./init.md) for initialization-specific prompts, decision points, and completion checks.

### 2. Ensure Icon Readiness
1. Check whether icon assets exist in the expected `src-tauri/icons` location.
2. If missing, generate a simple icon from random geometric primitives (for example circles, rectangles, triangles) with high contrast.
3. Export at least one source image (for example 1024x1024 PNG) and generate derived icon formats.
4. Re-run icon generation whenever branding changes.

Decision point:
- If no design assets are available yet, use generated geometric placeholders so app startup/build is not blocked.

### 3. Establish Frontend Modularity
1. Keep `App.tsx` as a shell/router/tab host only.
2. Split each major tab into `src/pages/*`.
3. Put reusable UI pieces under `src/components/*`.
4. Place shared helpers and state in `src/lib/*`.
5. Maintain global app state in `src/lib/state.ts`.

Quality check:
- `App.tsx` should not contain page-specific business logic or large inline components.

### 4. Establish Rust Modularity
1. Keep `lib.rs` focused on wiring and module exports.
2. Group Tauri commands in `src-tauri/src/commands/*` by domain.
3. Put cross-cutting helpers in `src-tauri/src/utils/*`.
4. Keep persistence-related code in `src-tauri/src/db/*`.
5. Use explicit module boundaries and small files.

Quality check:
- `lib.rs` should stay compact and avoid domain logic accumulation.

### 5. Implement Async Command Patterns
1. Mark I/O-bound Tauri commands `async`.
2. Use async crates and APIs for network and filesystem operations.
3. Avoid blocking calls in command handlers.
4. Return structured errors/messages suitable for UI handling.

Decision point:
- If a library is sync-only, isolate blocking work and document why async alternatives are not used.

### 6. Add Logging With Tracing
1. Configure `tracing` and a subscriber early in app startup.
2. Emit logs at `info` for user-visible actions, `debug` for deeper diagnostics.
3. Keep `info` logs concise: around 1 to 5 lines per user action.
4. Include contextual fields (command name, resource id, duration when practical).

Quality check:
- Logs should be useful for support/debugging without overwhelming output.

### 7. Final Verification Checklist
- Tauri version and config align with 2.0 conventions.
- `@tauri-apps/cli` exists in dev dependencies.
- `package.json` does not declare `packageManager`.
- Icons exist and app can start without icon-related failure.
- I/O-heavy commands are async.
- `App.tsx` and `lib.rs` are slim and delegated.
- `pages`, `components`, `lib`, `commands`, `utils`, and `db` folders are used as intended.
- Tracing logs are present and concise at `info` level.

## Example Prompts
- "Create a Tauri 2 app with three tabs using this skill's folder structure."
- "Refactor my current Tauri app so `App.tsx` and `lib.rs` are thin, and move logic into proper modules."
- "Audit this Tauri project for async command usage and tracing quality based on this skill."
