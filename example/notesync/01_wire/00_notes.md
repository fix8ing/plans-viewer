---
status: locked
---

# Notes

A note on the wire is its path, its body, and the hash the client last saw. Titles are derived server-side from the first heading.

### Shape

```jsonc
{
  "path": "projects/roadmap.md",
  "body": "# Roadmap\n\n...",
  "base_hash": "a1b2c3"      // null on first push
}
```

### Decisions

- **Path is the id.** Renames are delete + create; the server links them by `base_hash`.
- **Body is verbatim.** No front-matter parsing on the wire — the server owns that.
