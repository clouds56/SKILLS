---
name: talk-first
description: One-turn alignment mode for requests that start with `talk:` or ask Codex to "talk first" before acting. Use when the user wants critical discussion, context gathering, code reading, web research, or clarifying questions before implementation; the mode lasts only for the triggering turn unless the next user prompt also starts with `talk:`.
---

# Talk First

Use this skill as a temporary alignment mode. The goal is shared understanding before action, especially when the user may be wrong, Codex may be wrong, or the surrounding context may be incomplete.

## Core Behavior

- Treat the current turn as discussion and alignment, not implementation.
- Do not edit files, create commits, stage changes, deploy, or run destructive actions during this turn.
- Reading code, inspecting local state, searching documentation, browsing the internet, and running non-mutating diagnostic commands are allowed when they help test assumptions.
- Ask concise questions when agreement depends on missing context. Prefer one to three high-leverage questions over a long questionnaire.
- Challenge assumptions explicitly. Classify likely uncertainty as user-side, agent-side, source/context-side, or genuinely ambiguous.
- Separate facts from inferences. Name what was verified, what is suspected, and what still needs confirmation.
- End with either a proposed shared understanding, a small decision point for the user, or a concrete next-step recommendation.

## One-Turn Scope

The mode applies only to the prompt that triggered this skill.

- If the next user prompt starts with `talk:`, continue in talk-first mode for that new turn.
- If the next user prompt does not start with `talk:`, return to normal execution behavior and act on the aligned plan when appropriate.
- If the user says "talk first, then implement" in one prompt, complete the talk-first alignment in this turn and wait for the next prompt before making changes.

## Alignment Workflow

1. Restate the question or intended change in neutral terms.
2. Identify assumptions that could be wrong or underspecified.
3. Gather evidence only as needed:
   - Read relevant code or docs for local implementation questions.
   - Search current sources for unstable, external, legal, medical, financial, software, or product facts.
   - Ask the user when the missing context is preference, intent, access, or business logic that cannot be inferred safely.
4. Compare plausible interpretations and tradeoffs.
5. Recommend the most likely path, including what would change once normal execution resumes.

## Response Shape

Keep the response conversational and compact:

- Start with the aligned interpretation.
- Include the evidence or reasoning that matters.
- List open questions only if they block agreement.
- Avoid large implementation plans unless the user explicitly asks for one.

When uncertainty remains, say what would make the decision clearer rather than pretending the context is settled.
