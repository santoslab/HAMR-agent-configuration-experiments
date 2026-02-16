# Interaction Logs

This folder stores logs of Claude Code interactions for study by the research team.

## How to Save Logs

### After each session

1. **Export a terminal transcript**: Run `/export` in Claude Code at the end of a session. Move the resulting file to this folder with a descriptive name, e.g.:
   ```
   interaction-logs/2026-02-15-folder-structure-docs.txt
   ```

2. **Copy the raw JSONL log** (for programmatic analysis): The raw conversation data is stored automatically at:
   ```
   ~/.claude/projects/-Users-hatcliff-Dev-Claude-HAMR-Claude-configuration-experiments-git/<session-id>.jsonl
   ```
   Copy the relevant `.jsonl` file to this folder after a session:
   ```bash
   cp ~/.claude/projects/-Users-hatcliff-Dev-Claude-HAMR-Claude-configuration-experiments-git/*.jsonl interaction-logs/
   ```

## Naming Convention

Use the format: `YYYY-MM-DD-<brief-description>.<ext>`

Examples:
- `2026-02-15-mcp-tools-and-folder-docs.txt` (terminal transcript)
- `2026-02-15-mcp-tools-and-folder-docs.jsonl` (raw JSONL)

## File Formats

### Terminal Transcript (`.txt` from `/export`)

A text capture of the terminal session as it appeared in Claude Code. Contains Unicode box-drawing characters and symbols from the terminal UI (e.g., `▗ ▗ ▖ ▖` for the logo, `❯` for prompts, `⏺` for responses, `⎿` for tool result trees). Despite the `.md` extension that `/export` may assign, these are plain text files -- not clean Markdown.

**Best for:** Human review -- quickly reading the conversation flow, seeing which tools were called, and understanding the back-and-forth between user and Claude.

### Raw JSONL (`.jsonl` from `~/.claude/projects/`)

Structured JSON Lines format where each line is a JSON object representing a message in the conversation. Contains full detail: user messages, assistant responses, tool call requests, tool results, timestamps, token counts, and other metadata. Automatically recorded by Claude Code for every session.

**Best for:** Programmatic analysis -- parsing with scripts, extracting metrics (token usage, tool call frequency, response patterns), or building datasets for research.

### Comparison

| Aspect | Terminal Transcript (`.txt`) | Raw JSONL (`.jsonl`) |
|--------|------------------------------|----------------------|
| Source | `/export` command | Automatic (`~/.claude/projects/`) |
| Format | Plain text with Unicode UI characters | Structured JSON, one object per line |
| Readability | Human-readable (as seen in terminal) | Requires parsing/tooling |
| Content | Conversation text + abbreviated tool summaries | Full messages, tool calls, tool results, metadata |
| Best for | Quick review, sharing with team | Systematic/programmatic analysis |

We recommend saving **both formats** for each session to support different analysis needs.
