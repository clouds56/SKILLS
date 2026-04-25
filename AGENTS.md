# AGENTS.md

This repository is organized as a skill catalog.

## Repository Rule

- Every top-level folder in this repo is one separate skill.
- Do not combine multiple skills in a single folder.
- Keep skill names short, lowercase, and hyphenated.

## Expected Skill Folder Contract

Each skill folder should contain:

- `SKILL.md`: Required. Entry point with frontmatter (`name`, `description`) and usage instructions.
- Optional assets used by the skill (templates, scripts, examples, docs).

Recommended structure:

```text
<skill-name>/
  SKILL.md
  templates/        # optional
  scripts/          # optional
  examples/         # optional
```

## Discovery And Scope

- Skills are discovered by folder.
- If a folder has a valid `SKILL.md`, it is treated as an available skill.
- Keep each skill self-contained to avoid cross-skill coupling.

## Current State

- No skill folders are present yet.
