---
name: refactor-by-design
description: 'Plan-first module refactoring workflow. Use for distilling public contracts, revising design plans, architecture-first refactors, and clean rewrites with explicit breaking-change communication.'
argument-hint: 'Describe the module and target behavior changes.'
user-invocable: true
disable-model-invocation: false
---

# Refactor By Design

Use this skill to redesign a module from its external contract inward instead of patching internals incrementally.

## When To Use
- You want a clean-slate refactor with minimal coupling to legacy internals.
- The current module structure is hard to evolve, test, or reason about.
- You are willing to accept breaking changes when architecture requires it.
- You want a documented plan before code edits.

## Inputs
- Target module path and ownership boundary.
- User-visible capabilities the module must provide.
- Constraints: performance, compatibility, migration timeline, and risk tolerance.

## Outputs
- `<module>.plan.md` with external-facing features and key API/struct contracts.
- Updated plan after explicit review and intentional changes.
- Refactor architecture plan with breaking-change notes.
- Rewritten implementation replacing legacy code.

## Workflow

### 1) Distill
Create `<module>.plan.md` focused on what the module does for callers.

Include:
- Core feature/workflow list the module must provide.
- Contract of KEY APIs/structs only (function signatures, data contracts, invariants, error model).
- Explicit non-goals and deferred details.

Rules:
- Prioritize outside-facing APIs over internal representation.
- Avoid documenting every private detail.
- Separate required behavior from implementation ideas.

Completion checks:
- Every major caller use-case maps to a feature entry.
- KEY APIs/structs have contracts clear enough for reimplementation.
- Plan can be reviewed without reading source internals.

### 2) Modify Plan
Revise the plan before touching implementation.

Include:
- Scope adjustments and missing requirements.
- Clarified contracts and acceptance criteria.
- Trade-off decisions and rationale.

Rules:
- Do not consult current implementation unless explicitly requested.
- Treat this as design-first work, not reverse engineering.
- Preserve focus on external behavior and compatibility intent.

Completion checks:
- Ambiguities are resolved or marked as open questions.
- Acceptance criteria are testable.
- The plan is approved for implementation.

### 3) Refactor
Review current implementation as a whole, then design the architectural refactor path.

Include:
- Whole-module assessment: coupling, layering, ownership, and risk hotspots.
- Target architecture and migration strategy.
- Breaking-change list and user impact statement.

Rules:
- Prefer architectural coherence over incremental patching.
- Breaking changes are allowed; communicate them explicitly and early.
- Define checkpoints for behavior parity where required.

Completion checks:
- Refactor plan identifies major structural moves and sequence.
- Breaking changes are documented with migration notes.
- Risks and rollback/verification strategy are captured.

### 4) Rewrite
Delete the old implementation and rebuild from the approved plan.

Include:
- New module structure aligned with the target architecture.
- New API/struct implementations matching plan contracts.
- Tests or checks that validate the promised behaviors.

Rules:
- Replace legacy code instead of preserving old shape.
- Keep code organized around responsibilities from the plan.
- Reject scope creep not captured in the approved plan.

Completion checks:
- Legacy implementation is removed.
- New code satisfies all contract and acceptance checks.
- Breaking-change notes and migration guidance are published.

## Decision Points
- If external contract is unclear: pause and expand `Distill` before refactoring.
- If compatibility requirements tighten: revise `Modify Plan` and re-baseline acceptance criteria.
- If rewrite risk is high: tighten acceptance checks and migration notes, but keep the hard rewrite approach.

## Quality Bar
- External API clarity over internal detail density.
- Design intent is documented before implementation.
- Breaking changes are explicit, not accidental.
- Final code structure matches the approved design.

## Example Prompts
- "Use refactor-by-design on `auth/session` and produce `auth-session.plan.md` first."
- "Run only steps 1 and 2 for `payments/ledger`; do not inspect implementation yet."
- "Apply refactor-by-design to `notifications` and include a breaking-change migration note."
