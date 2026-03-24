---
name: fastmail
description: Email management via fastmail-cli — read, search, compose, triage, contacts, and masked email for jfd@thermopylae.com. Invoke when the user asks about email, messages, inbox, or contacts.
allowed-tools: Bash
---

# fastmail-cli — Complete Reference

fastmail-cli is a Rust CLI for Fastmail via JMAP (email) and CardDAV (contacts). All output is JSON: `{"success": true, "data": {...}}`. The user's account is **jfd@thermopylae.com**.

## Safety Rules

### Compose Operations Require Confirmation

Before executing `send`, `reply`, or `forward` without `--draft`: display the full command to the user — including all recipients, subject, and body — and ask "Should I send this?" Wait for explicit yes.

When intent is ambiguous ("write a reply", "draft an email"), default to `--draft`. Only omit `--draft` when the user clearly intends to send immediately.

### Mass-Destructive Operations Are Forbidden

**Never** iterate over search or list results to apply destructive operations to multiple emails. No loops, xargs pipelines, or shell scripts that apply `send`, `reply`, or `forward` to multiple items. If the user asks for bulk sending, explain this must be done via the Fastmail web interface.

### Never Skip CLI Confirmation Prompts

Never pass `-y` or `--yes` to bypass the CLI's built-in confirmation prompts.

## Setup

```bash
fastmail-cli auth fmu1-YOUR-TOKEN
```

Config lives at `~/.config/fastmail-cli/config.toml`:
```toml
[core]
api_token = "fmu1-..."

[contacts]
username = "you@fastmail.com"
app_password = "xxxx..."
```

Or via env: `FASTMAIL_API_TOKEN`, `FASTMAIL_USERNAME`, `FASTMAIL_APP_PASSWORD`

Debug: `RUST_LOG=debug fastmail-cli [cmd]`

---

## Command Reference

### List

```bash
fastmail-cli list emails [-m MAILBOX] [-l LIMIT]     # default: INBOX, 50
fastmail-cli list mailboxes
fastmail-cli list identities                          # sender aliases for --from
```

### Get & Thread

```bash
fastmail-cli get EMAIL_ID                            # full email with body
fastmail-cli thread EMAIL_ID                         # entire conversation
```

### Search

```bash
fastmail-cli search [OPTIONS]
  --text/-t STR       # full-text (from/to/subject/body)
  --from/--to/--cc/--bcc/--subject/--body STR
  --mailbox/-m STR
  --before/--after STR  # ISO 8601: 2024-01-15
  --unread --flagged --has-attachment
  --min-size/--max-size BYTES
  --limit/-l N        # default 50
```

### Compose

```bash
fastmail-cli send --to ADDR --subject SUBJ --body BODY [--cc] [--bcc] [--from IDENTITY] [--draft]
fastmail-cli reply EMAIL_ID --body BODY [--all] [--cc] [--bcc] [--from IDENTITY] [--draft]
fastmail-cli forward EMAIL_ID --to ADDR [--body STR] [--cc] [--bcc] [--from IDENTITY] [--draft]
```

### Manage

```bash
fastmail-cli move EMAIL_ID --to MAILBOX
fastmail-cli mark-read EMAIL_ID [--unread]
fastmail-cli spam EMAIL_ID
```

### Attachments

```bash
fastmail-cli download EMAIL_ID [-o OUTPUT_DIR] [-f raw|json] [--max-size 1M]
```

### Masked Email

```bash
fastmail-cli masked list
fastmail-cli masked create [--domain URL] [--description STR] [--prefix STR]
fastmail-cli masked enable/disable/delete ID [-y]
```

### Contacts

```bash
fastmail-cli contacts list
fastmail-cli contacts search QUERY    # name, email, or org
```

### Other

```bash
fastmail-cli completions bash|zsh|fish|powershell
fastmail-cli mcp    # start MCP server for Claude Desktop
```

---

## Common Patterns

```bash
# Find unread emails from a sender
fastmail-cli search --from boss@company.com --unread

# Get a thread then reply
fastmail-cli thread abc123
fastmail-cli reply abc123 --body "Thanks, will do." --from work@me.com

# Save draft instead of sending
fastmail-cli send --to x@y.com --subject "Draft" --body "..." --draft

# Download all attachments from an email
fastmail-cli download abc123 -o ~/Downloads

# Move to folder after reading
fastmail-cli move abc123 --to "Archive"
```

---

## Subcommand Skills

- `/fastmail/search` — search workflows and filter combinations
- `/fastmail/compose` — send, reply, forward, drafts, identities
- `/fastmail/conversations` — threading, listing, reading
- `/fastmail/attachments` — downloading and extracting attachments
- `/fastmail/masked` — masked email management
- `/fastmail/contacts` — contact search
