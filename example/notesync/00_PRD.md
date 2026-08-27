---
status: locked
---

# PRD

`notesync` mirrors a folder of markdown notes to the hosted API. The folder is the source of truth; the server stores history and serves search.

> Anything the server can decide from the diff, the CLI must not ask about.

## Goals

- **Folder = desired state** — every push sends the full tree. Deleted locally means deleted remotely. No partial pushes, no `--force`.
- **No prompts** — conflicts resolve by rule (see `01. Wire / Conflicts`), never by a question. Runs clean in CI.
- **Generated client** — types and requests come from the OpenAPI spec. Nothing hand-written touches the wire.
- **Offline first** — every command that can run without the network does.

## Not doing (v0)

- Attachments larger than 25 MB.
- Shared folders — one owner per tree.
- A GUI.
