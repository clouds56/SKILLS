---
name: github-worktree
description: Manage GitHub PR workflows from a git worktree with `gh` and `git`. Use when the user is working inside a worktree and asks to start a PR, create an empty PR stub, decide whether the user or the agent manages commits, push branch updates, or refresh PR titles and descriptions.
---

# GitHub Worktree

Use this skill to manage branch, commit, push, and PR flow from the current git worktree without losing track of ownership.

## Workflow

### 1. Confirm worktree context

Verify that the current directory is a git worktree before discussing PR creation.

Check:
- Current branch name.
- Current worktree path.
- Repository root.
- GitHub CLI availability and auth state when PR work is requested.

If the directory is not a worktree checkout, say so plainly and continue only if the user still wants normal branch-based PR handling.

### 2. Clarify the PR request

When the user says "start a PR" or equivalent, ask only the missing questions needed to proceed:
- Target base branch.
- PR title, formatted as a Conventional Commits subject such as `feat: add X`, `fix: correct Y`, or `test: cover Z`.
- PR description or the points that should be included in it.
- Whether commit management is `user` or `agent`.

Always ask who manages commits.

Interpret the answer as:
- `user`: The user owns normal commit boundaries and commit content. The agent must not create or push normal work commits without an explicit follow-up request, but may create a single empty bootstrap commit when needed to open the PR stub.
- `agent`: The agent owns normal commit boundaries, may create bootstrap commits when needed, and may push to the remote branch as part of normal progress.

### 3. Check whether the branch already has a PR

Before creating a new PR, check whether the current branch already has any remote PRs, including closed PRs.

Use `gh` in a way that includes all states, not only open PRs.

If any PR exists for the branch:
- Show the PR number, state, title, and URL.
- Ask the user what to do next.
- Do not silently create a replacement PR.

Typical user choices:
- Reuse the existing open PR.
- Reopen or supersede a closed PR.
- Create a fresh branch and new PR.

### 4. Create an empty PR stub

If no PR exists and the user wants to start the PR before implementation, create an empty PR stub after the title and description are clear.

Important constraint:
- GitHub cannot open a PR from a branch with no divergence from base.

Handle that constraint as follows:
- If commits are `agent` managed and the branch has no unique commits, create an empty bootstrap commit with `git commit --allow-empty`.
- Push the branch to the remote.
- Create the PR with `gh pr create` using the agreed title, body, head branch, and base branch.

If commits are `user` managed and the branch has no unique commits:
- Explain that a placeholder PR still needs branch divergence.
- Create a single empty bootstrap commit only for PR creation.
- Push the branch and create the PR stub.
- Do not treat that bootstrap commit as permission to manage later commits.

Prefer a draft PR unless the user explicitly asks for a ready-for-review PR.

After creating the PR stub, continue with the actual work instead of stopping at PR creation:
- If the problem is not yet understood, start investigating.
- If the task needs up-front structure, produce or confirm a plan.
- If the next implementation step is already clear, proceed directly.

### 5. Manage commits and pushes

If commit ownership is `agent`:
- Commit logically grouped changes when the work reaches a stable checkpoint.
- Push after commits when remote state should stay in sync.
- Use clear commit messages tied to the work being performed.

If commit ownership is `user`:
- Leave normal commit boundaries and content to the user.
- Do not create or push normal work commits unless the user explicitly asks.
- Allow one exception: create and push a single empty bootstrap commit if branch divergence is required to open the initial PR stub.
- After the PR stub exists, return to user-managed commit boundaries unless the user changes ownership.

### 6. Keep the PR description current

When the agent pushes new work, or when the user asks to update the PR text, refresh the PR title and description to match the current scope.

Update the PR when:
- Scope changes materially.
- New implementation details should be reflected.
- The original placeholder description is stale.

Keep PR descriptions concise and operational:
- Problem or goal.
- What changed.
- Testing or validation status.
- Any open questions or follow-ups.

Keep the PR title aligned with Conventional Commits semantics when the scope changes enough that the original type or summary is no longer accurate.

### 7. Report state clearly

After any PR-related action, report:
- Branch name.
- Whether the branch was pushed.
- Whether a PR already existed or was created.
- PR number and URL when available.
- Whether commit ownership is currently `user` or `agent`.

## Guardrails

- Do not create a PR before checking for existing open or closed PRs on the branch.
- Do not assume the agent may commit. Ask first.
- Do not create or push normal work commits under `user` ownership without explicit permission; only the bootstrap empty commit for initial PR creation is allowed.
- Do not leave the PR description stale after agent-driven pushes when the summary has changed.
- Do not use arbitrary PR titles; use a Conventional Commits type and concise subject.
- Do not hide GitHub constraints around empty PRs; explain the need for at least one unique commit.

## Example Prompts

- "Use $github-worktree and start a PR for this worktree."
- "Use $github-worktree to check whether this branch already had a PR."
- "Use $github-worktree and create a draft PR stub, agent-managed commits."
- "Use $github-worktree to update the PR description after my latest push."
