# fastmail-cli

You have access to `fastmail-cli`, a CLI for Fastmail's JMAP API. Use it via bash. All commands output JSON.

Do NOT use the MCP server — use the CLI directly. It's faster and doesn't bloat context with tool schemas.

## Quick Reference

### Reading
```
fastmail-cli list mailboxes                           # folders + unread counts
fastmail-cli list emails [--mailbox NAME] [--limit N] # list emails (default: INBOX, 50)
fastmail-cli list identities                          # sender addresses (for --from)
fastmail-cli get EMAIL_ID                             # full email + thread context
fastmail-cli thread EMAIL_ID                          # all emails in conversation
fastmail-cli search [filters] [--limit N]             # search (default: 50)
```

### Search Filters (all ANDed)
`--text` (full-text), `--from`, `--to`, `--cc`, `--bcc`, `--subject`, `--body`, `--mailbox`, `--has-attachment`, `--min-size`, `--max-size`, `--before YYYY-MM-DD`, `--after YYYY-MM-DD`, `--unread`, `--flagged`

### Sending
```
fastmail-cli send --to ADDRS --subject TEXT --body TEXT [--cc ADDRS] [--bcc ADDRS] [--from IDENTITY]
fastmail-cli reply EMAIL_ID --body TEXT [--all] [--cc ADDRS] [--bcc ADDRS] [--from IDENTITY]
fastmail-cli forward EMAIL_ID --to ADDRS [--body TEXT] [--cc ADDRS] [--bcc ADDRS] [--from IDENTITY]
```
Multiple recipients: comma-separated. Use `list identities` to find valid `--from` values.

### Actions
```
fastmail-cli move EMAIL_ID --to MAILBOX    # Archive, Trash, etc.
fastmail-cli mark-read EMAIL_ID [--unread] # mark read/unread
fastmail-cli spam EMAIL_ID -y              # mark spam + train filter (DESTRUCTIVE)
```

### Attachments
```
fastmail-cli download EMAIL_ID [--output DIR] [--format json] [--max-size 500K]
```
`--format json` extracts text from PDFs, DOCX, etc. (56 formats). `--max-size` resizes images.

### Contacts (requires FASTMAIL_APP_PASSWORD, not API token)
```
fastmail-cli contacts list
fastmail-cli contacts search QUERY         # search by name/email/org
```

### Masked Email
```
fastmail-cli masked list
fastmail-cli masked create [--domain URL] [--description TEXT] [--prefix PREFIX]
fastmail-cli masked enable ID
fastmail-cli masked disable ID
fastmail-cli masked delete ID -y           # permanent!
```

## Safety Rules
- NEVER send/reply/forward without showing the user what will be sent and getting explicit approval
- NEVER mark as spam without user confirmation (trains the filter — affects future mail)
- NEVER delete masked emails without user confirmation (permanent)

## Output Format
All commands return: `{"success": bool, "data": ..., "message": "...", "error": "..."}`
Use `jq` for extraction, e.g.: `fastmail-cli list mailboxes | jq '.data[]'`
