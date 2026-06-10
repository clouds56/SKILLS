---
name: review-pr
description: Review a GitHub pull request from a clean, detached local worktree. Use when asked to review a PR for code cleanliness, design quality, test coverage, regressions, or corner cases without modifying the user's active checkout.
---

# Review PR

Use this skill to perform a code-review pass on a GitHub pull request. The review target must be a clean, detached worktree for the PR head, not the user's active checkout.

## Review Priorities

Lead with concrete findings. Prioritize:

- **Correctness and corner cases**: behavior changes, edge cases, error paths, concurrency, data loss, migrations, security, permissions, compatibility, and rollback risks.
- **Design quality**: whether the change fits existing architecture, ownership boundaries, naming, data flow, APIs, abstractions, and failure handling.
- **Code cleanliness**: readability, maintainability, duplication, unnecessary complexity, dead code, local style, and consistency with nearby code.
- **Test coverage**: whether tests cover the changed behavior, negative cases, integration points, and important regressions.

Avoid praising routine code. Do not list non-blocking style preferences unless they affect maintainability or future defects.

## Workspace Setup

Never review directly in the user's active worktree. It may contain unrelated edits, stale branches, or a branch already checked out elsewhere.

Use `gh` to resolve PR metadata, then use `git` for the exact checkout and diff:

```bash
gh pr view <pr> --json number,title,url,baseRefName,baseRefOid,headRefName,headRefOid,headRepositoryOwner,headRepository
```

Create a project-local review workspace:

```text
.review-pr/<pr-number>/<base-sha>-<head-sha>/head
.review-pr/<pr-number>/<base-sha>-<head-sha>/base
```

The `base` worktree is optional. Create it only when you need to reproduce base behavior, compare generated output, or run base tests.

Fetch refs as needed, then create the head worktree detached at the exact PR head commit:

```bash
git fetch origin <base-ref>
git fetch <head-remote> <head-ref>
git worktree add --detach .review-pr/<pr-number>/<base-sha>-<head-sha>/head <head-sha>
```

If the head repository is a fork, derive a temporary remote name from the owner or URL, add it only if needed, and fetch the PR head from that remote. Do not check out a branch name for review; branches can be stale, already checked out, or locally divergent.

If `.review-pr/` is not ignored, do not silently edit `.gitignore` during a review. Warn the user before creating files there, or ask permission if the review cannot proceed safely.

## Diff Source Of Truth

Use local git diff from inside the detached head worktree. Treat the three-dot diff as the review scope:

```bash
git diff --name-status <base-sha>...HEAD
git diff --find-renames <base-sha>...HEAD
git diff --check <base-sha>...HEAD
```

Inspect changed files in full when needed. A hunk can hide a bug caused by unchanged surrounding code, but findings should be tied to the PR's behavioral impact.

Use per-file diffs for focused inspection and line references:

```bash
git diff --find-renames <base-sha>...HEAD -- <path>
```

Do not rely on GitHub-rendered patches as the primary source once the local checkout exists. Local `git diff` is more reliable for rename detection, tooling, test selection, and repeated inspection.

## Review Workflow

1. Inspect PR metadata and create the detached review workspace.
2. Read the changed-file list and classify the blast radius.
3. Inspect diffs and surrounding code for correctness, design, cleanliness, tests, and corner cases.
4. Run targeted validation when practical. Prefer existing project commands and tests over invented checks.
5. If tests are missing or too narrow, identify the specific uncovered behavior.
6. Report findings first, ordered by severity, with file and line references.

When test commands are expensive, unavailable, or require network access, state what was and was not run.

## Output Format

Use a code-review stance:

- Start with findings, ordered by severity.
- Include file and line references for each actionable issue.
- Explain the risk and the condition that triggers it.
- Suggest the smallest better approach when useful.
- After findings, add open questions or assumptions.
- Keep the summary brief and secondary.

If there are no findings, say so clearly and mention any residual test or coverage risk.

## Guardrails

- Do not modify the PR checkout unless the user explicitly asks for fixes.
- Do not modify the user's active worktree.
- Do not create, switch to, or update local branches for the PR head.
- Do not treat generated or vendored diffs as primary review material unless the PR changes that source of truth.
- Do not let formatting or style comments crowd out correctness, design, tests, or corner cases.
- Do not claim tests passed unless the command completed successfully in the relevant review workspace.
