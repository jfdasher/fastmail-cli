# fastmail-cli Skills Fork

This is jfdasher's fork of radiosilence/fastmail-cli, maintained primarily to host customized Claude Code skills for email management via `fastmail-cli`.

## Account

The configured Fastmail account is **jfd@thermopylae.com**.

## Skills

Skills live in `.claude/skills/` and are symlinked into `~/.claude/skills/` for global availability. The parent skill is `fastmail.md` with sub-skills in `fastmail/`.

### Guardrail Policy

- **Compose operations** (send, reply, forward) require user confirmation before executing. Ambiguous compose requests default to `--draft`.
- **Triage operations** (move, spam, mark-read) are ungated — the user wants these usable in bulk without per-item confirmation.
- **Mass sending is forbidden** — no loops or pipelines that send/reply/forward to multiple items.
- The `-y` flag should not be passed to skip CLI confirmation prompts.

## Permissions

Global permissions in `~/.claude/settings.json` auto-allow all read and triage commands. `send`, `reply`, and `forward` are deliberately omitted so Claude Code's permission system acts as the confirmation gate.
