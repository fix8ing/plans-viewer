---
status: locked
---

# PR stacks

Two stacks; 1 gates 2. Each PR is one bullet — small enough to review in a sitting.

### Stack 1 — Server

- 1.1 `PUT /tree` accepting a manifest; per-note endpoint stays for now.
- 1.2 Conflict rule from `01. Wire / Conflicts`, with the `.conflict.md` write.
- 1.3 Delete-on-absence: a path missing from the manifest is a delete.

### Stack 2 — Client

- 2.1 Generated client from the spec; hand-written requests deleted.
- 2.2 `notesync push` — hash tree, send manifest, print the report.
- 2.3 `notesync pull` — fetch, write, never overwrite a dirty file.
- Not: attachments (draft), shared folders.
