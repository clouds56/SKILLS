---
name: github-worktree
description: Manage GitHub PR workflows from a git worktree with `gh` and `git`. Use when the user is working inside a worktree and asks to start a PR, create an empty PR stub, decide whether the user or the agent manages commits, push branch updates, or refresh PR titles and descriptions.
---

# GitHub Worktree

Use this skill to manage branch, commit, push, and PR flow from the current git worktree without losing track of ownership.

For any question to the user about repository, branch, PR, commit, push, or git state, use the ask/question tool instead of a plain chat question.

## Workflow

### Useful command patterns

Use these as examples and adapt flags to the repository state.

```bash
# confirm worktree and branch context
git worktree list
git branch --show-current
git rev-parse --show-toplevel

# check gh availability and auth
gh --version
gh auth status

# inspect repo owner/name and default branch
gh repo view --json nameWithOwner,defaultBranchRef

# check whether the current branch already has any PR, including closed ones
branch="$(git branch --show-current)"
gh pr list --head "$branch" --state all

# check whether the branch already differs from the chosen base
git rev-list --left-right --count origin/main..."$branch"

# create the bootstrap commit when branch divergence is required
git commit --allow-empty -m "chore: bootstrap PR"

# push the current branch
git push -u origin "$branch"

# prepare the PR metadata directory before writing a PR body file
mkdir -p .github/_pr_

# create a draft PR stub
gh pr create --draft --base main --head "$branch" --title "feat: add X" --body-file .github/_pr_/body.md

# persist PR status after PR creation
python3 scripts/write_pr_status.py

# update PR title and description later
gh pr edit --title "fix: correct Y" --body-file .github/_pr_/body.md
```

### 1. Inspect repository and worktree state

Inspect repository state before asking any PR questions through the ask/question tool.

Check:
- Current branch name.
- Repository default branch, using `gh repo view --json nameWithOwner,defaultBranchRef`.
- Repository name.
- Existing PR status for the current branch, including closed PRs.
- Current worktree path.
- Repository root.
- GitHub CLI availability and auth state when PR work is requested.

If the output includes `X Failed to log in to github.com account` and/or `The token in default is invalid.`, treat that as a likely sandbox or credential-isolation artifact for Codex rather than a reliable signal that the repository workflow itself must stop.
Even when `gh auth status` reports invalid credentials in that sandbox-shaped way, go ahead and use `gh pr view`, `gh pr list`, and `gh pr create` if the workflow calls for them.

If `git branch --show-current` returns an empty string, treat the checkout as a detached HEAD.
In that case:
- Do not look for PRs on the empty branch name.
- Resolve the repository default branch first.
- Use the first PR question through the ask/question tool to ask which new branch name to create.
- After the user answers, run `git fetch origin`, reset the worktree to `origin/<default-branch>`, and create the requested branch before continuing PR setup.

If the directory is not a worktree checkout, say so plainly and continue only if the user still wants normal branch-based PR handling.

If HEAD is attached to a branch and any PR exists for that branch:
- Show the PR number, state, title, and URL.
- Ask what to do next through the ask/question tool.
- Do not silently create a replacement PR.

Typical user choices:
- Reuse the existing open PR.
- Reopen or supersede a closed PR.
- Create a fresh branch and new PR.

### 2. Ask the three PR questions

When the user says "start a PR" or equivalent, ask these three questions through the ask/question tool before creating anything.

If HEAD is attached to a branch, ask:
- Which branch should merge into which branch. If the target branch is not specified, resolve the repository default branch first with `gh repo view --json nameWithOwner,defaultBranchRef` and confirm against that.
- Whether commit management is `user` or `agent`.
- The PR title, formatted as a Conventional Commits subject such as `feat: add X`, `fix: correct Y`, or `test: cover Z`.

If HEAD is detached, replace the first question with:
- What branch name should be created from `origin/<default-branch>` for this PR.

Then:
- Reset the worktree to `origin/<default-branch>`.
- Create and check out the requested branch.
- Continue with the remaining two questions.

After branch creation in detached-head mode, treat the merge target as the repository default branch unless the user explicitly asks for a different base branch.

Treat confirmation of the PR title as permission to create the PR.

Interpret the answer as:
- `user`: The user owns normal commit boundaries and commit content. The agent must not create or push normal work commits without an explicit follow-up request, but may create a single empty bootstrap commit when needed to open the PR stub.
- `agent`: The agent owns normal commit boundaries, may create bootstrap commits when needed, and may push to the remote branch as part of normal progress without asking again for commit or push permission.

### 3. Create an empty PR stub

If no PR exists and the user wants to start the PR before implementation, create an empty PR stub after the three questions are answered and the PR title is confirmed.

Important constraint:
- GitHub cannot open a PR from a branch with no divergence from base.

Handle that constraint as follows:
- Create an empty bootstrap commit with `git commit --allow-empty`.
- Push the branch to the remote.
- Create the PR with `gh pr create` using the confirmed title, the current branch as head, and the confirmed target branch as base.
- Use a concise placeholder PR body if a fuller description is not available yet.

In detached-head mode, finish the branch recreation flow before checking divergence or creating the bootstrap commit. The bootstrap commit must happen on the newly created branch, not on the detached commit.

This bootstrap commit is allowed under both ownership modes only for starting the PR.

Prefer a draft PR unless the user explicitly asks for a ready-for-review PR.

### 4. Persist PR status

Immediately after creating the PR stub, write `.github/_pr_/status.json` and `.github/_pr_/.gitignore`.

Use `scripts/write_pr_status.py` to:
- Discover the current branch.
- Discover the repository name and default branch.
- Discover PR state for the current branch.
- Create `.github/_pr_/.gitignore` containing only `*`.
- Write `.github/_pr_/status.json` with the discovered data.

### 5. Start the plan

After the PR stub exists and the status file is written, start the plan for the actual work.

### 6. Manage commits and pushes

If commit ownership is `agent`:
- Commit logically grouped changes when the work reaches a stable checkpoint.
- Since PRs are squash-merged, do not try to keep the branch as one coherent commit. Prefer reviewable working commits and let the squash merge create the final single commit.
- Before pushing, consider current CI status when it is available from the branch or PR context.
- If CI is already failing for a known reason unrelated to the new work, avoid noisy pushes unless the push is part of addressing that failure or the user explicitly wants the remote updated anyway.
- If the new work is meant to fix an existing CI failure, pushing is expected so the fix can be validated remotely.
- Push after commits when remote state should stay in sync and the CI situation has been considered.
- Do not ask again for permission to commit or push once ownership is `agent`.
- If a bootstrap empty commit was used to open the PR stub, it may be amended into the first actual work commit or left in history until the PR is squash-merged. Choose the simpler path for the current state.
- Use clear commit messages tied to the work being performed.

If commit ownership is `user`:
- Leave normal commit boundaries and content to the user.
- Do not create or push normal work commits unless the user explicitly asks.
- Allow one exception: create and push a single empty bootstrap commit if branch divergence is required to open the initial PR stub.
- After the PR stub exists, return to user-managed commit boundaries unless the user changes ownership.

### 7. Keep the PR description current

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

If a PR body file must be written, write it to `.github/_pr_/body.md`. Do not write PR body markdown to `/tmp`.

Keep the PR title aligned with Conventional Commits semantics when the scope changes enough that the original type or summary is no longer accurate.

When the work is complete and the PR is no longer intended to stay in draft, make the draft PR ready for review.
- Do this after the implementation is ready, the PR description is current, and any expected validation state has been reported.
- Use `gh pr ready` unless the user explicitly wants to keep the PR in draft.

### 8. Report state clearly

After any PR-related action, report:
- Branch name.
- Default branch.
- Repository name.
- Whether the branch was pushed.
- Whether a PR already existed or was created.
- PR number and URL when available.
- Whether commit ownership is currently `user` or `agent`.

## Guardrails

- Do not create a PR before checking for existing open or closed PRs on the branch.
- Do not ask repo, PR, or git-related questions in plain chat. Use the ask/question tool.
- Do not treat a detached HEAD as a usable branch name. Ask for a branch name first through the ask/question tool, then reset to `origin/<default-branch>` and create that branch before PR creation.
- Do not assume the agent may commit before commit ownership is established. Use the ask/question tool to establish ownership first, then follow it: `agent` means the agent should commit and push as needed for normal progress, while `user` means normal commits stay user-managed.
- Do not create or push normal work commits under `user` ownership without explicit permission; only the bootstrap empty commit for initial PR creation is allowed.
- Do not ask again for commit or push permission after the user has chosen `agent` ownership.
- Do not skip writing `.github/_pr_/status.json` after PR stub creation.
- Do not leave the PR description stale after agent-driven pushes when the summary has changed.
- Do not write PR body markdown to `/tmp`; use `.github/_pr_/body.md`.
- Do not treat `X Failed to log in to github.com account` or `The token in default is invalid.` from `gh auth status` as definitive user-facing auth failures; these are common signs of the sandbox or isolated Codex credentials.
- Go ahead with `gh pr view`, `gh pr list`, and `gh pr create` when the workflow needs them, even if `gh auth status` reports those sandbox-shaped credential errors.
- Do not ignore known failing CI before pushing. Consider whether the push will help resolve the failure or only add noise.
- Do not use arbitrary PR titles; use a Conventional Commits type and concise subject.
- Do not hide GitHub constraints around empty PRs; explain the need for at least one unique commit.

## Example Prompts

- "Use $github-worktree and start a PR for this worktree."
- "Use $github-worktree to check whether this branch already had a PR."
- "Use $github-worktree and create a draft PR stub, agent-managed commits."
- "Use $github-worktree to update the PR description after my latest push."
