# Scenario: Floating Window App

Use this profile for lightweight, always-on-top or tray-adjacent utilities built for quick access, transient interaction, and minimal screen footprint.

Examples: sticky note, mini translator, capture overlay.

## Architecture Defaults

1. Treat the floating window as a focused micro-surface, not a full workspace shell.
2. Keep startup fast and memory footprint low.
3. Default window style to undecorated and transparent, with a light gray content background for readability.
4. Default drag behavior to whole-window drag only for non-text utility surfaces; switch to handle-only drag when content needs selection or text input.
5. Keep command wrappers in `src/lib/tauri.ts` and shared payload/result types in `src/lib/types.ts`.
6. Keep state minimal and predictable; prefer local state unless persistence is clearly needed.

## Window Behavior Focus

1. Default to undecorated windows; only enable decorations when a platform-specific usability issue requires it.
2. Default to transparent windows with a light gray background layer inside the content area.
3. Keep drag behavior explicit and mutually exclusive: whole-window drag mode or header-handle drag mode.
4. Keep resize policy explicit: fixed size, bounded resize, or content-driven resize.
5. Validate always-on-top, focus behavior, and tray restoration paths.

## Decorations And Transparency Guidance

1. If using undecorated windows, provide custom close/minimize and drag affordances; permission `core:window:allow-close` is a MUST when your UI triggers close actions.
2. If using transparency, avoid unreadable text by defining contrast-safe surfaces.
3. On macOS, transparent-window behavior requires enabling `macOSPrivateApi`; this is a MUST when transparent windows are used.
4. Keep pointer events intentional for click-through overlays vs. interactive overlays.
5. Document platform differences for transparency, blur, and shadow behavior.

## Example: src-tauri/capabilities/default.json

Use this minimal capability set when your floating window needs drag support, close actions, and opener APIs.

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capability set for floating window behavior.",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:allow-start-dragging",
    "core:window:allow-close",
    "opener:default"
  ]
}
```

## Example: src-tauri/tauri.conf.json

Use this as a baseline for a transparent floating window on macOS.

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Floating Utility",
  "version": "0.1.0",
  "identifier": "com.example.floating-utility",
  "app": {
    "macOSPrivateApi": true,
    "windows": [
      {
        "label": "main",
        ...
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "resizable": false
      }
    ],
    "security": {
      "capabilities": ["default"]
    }
  }
}
```

Also enable the `macOSPrivateApi` feature for `tauri` in `src-tauri/Cargo.toml` when using "macOSPrivateApi" set to true.

## Drag, Drop, And Resize Guidance

1. Use one of two drag modes only:
2. Whole-window drag mode: make the complete content surface draggable, disable text selection (`user-select: none`), and avoid inline text-entry controls in that surface.
3. Header-handle drag mode: keep the app window undecorated (`decorations: false`) and provide a custom in-app header/handle that calls `startDragging()`.
4. Prefer calling `startDragging()` from your drag handle interaction instead of relying on `data-tauri-drag-region`, which is less reliable across app states and layouts.
5. For either drag mode, permission `core:window:allow-start-dragging` is a MUST.
6. Define drag-and-drop zones and reject states clearly.
7. Throttle resize/reflow loops when window size follows inner content.
8. Set min/max window bounds to avoid unusable micro or oversized states.
9. Recompute layout safely when content expands or collapses dynamically.

## Inner Content Interaction Guidance

1. Prioritize keyboard shortcuts and quick-dismiss interactions (Esc, Enter, global toggles).
2. Keep interaction depth shallow: quick action in one to three steps.
3. Avoid modal stacks in tiny windows; prefer inline panels.
4. Preserve cursor/focus behavior during auto-resize and reposition operations.
5. Ensure selection, copy/paste, and text input remain reliable under transparency and custom drag regions.

## Command And Data Model Guidance

1. Keep commands short-lived and idempotent where possible.
2. Emit compact, typed event payloads for transient UI updates.
3. Define lightweight model types such as `FloatingWindowState`, `WindowMode`, and `QuickActionResult`.
4. Promote to job/task scenarios only when long-running background work becomes core behavior.

## Common Overlaps

1. With simple-ui app when no persistence or background work is required.
2. With tooling app when quick actions wrap local CLI utilities.
3. With workspace app when notes/snippets become durable and organization-heavy.
4. With control panel app when the floating surface is an operator quick panel.
