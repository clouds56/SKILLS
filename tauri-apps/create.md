# Create Tauri App (Tauri 2.0, pnpm-first)

This document covers initialization and baseline setup.

## 1. Initialize

Preferred command:

```bash
pnpm create tauri-app
```

Choose a template (for example React + TypeScript) and confirm Tauri 2.x defaults.

## 2. Package Rules (`package.json`)

### Required
1. Use pnpm for install and scripts execution.
2. Keep `@tauri-apps/cli` in `devDependencies`.
3. Keep scripts that call Tauri CLI through local dependency.

### Forbidden
1. Do not write a `packageManager` field in `package.json`.

### Baseline snippet

```json
{
  "name": "my-tauri-app",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^2"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "typescript": "^5",
    "vite": "^5"
  }
}
```

## 3. Icons

Icons are required before first app run/build.

### Expected location
- `src-tauri/icons`

### Procedure
1. Check whether icon assets already exist.
2. If missing, generate a high-contrast placeholder from simple geometry (circles/rectangles/triangles).
3. Export one source image at least `1024x1024` PNG.
4. Generate derived Tauri icon formats into `src-tauri/icons`.
5. Regenerate when branding changes.

### Decision point
- If no design assets are ready, use geometric placeholders immediately so startup/build is not blocked.

## 4. Quick Verification

1. Tauri major version is 2.x.
2. `pnpm install` succeeds.
3. `@tauri-apps/cli` exists in `devDependencies`.
4. `package.json` has no `packageManager` key.
5. `src-tauri/icons` contains generated icons.
