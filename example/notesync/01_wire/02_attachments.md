---
status: draft
label: Attachments & blobs
---

# Attachments

Binary files next to notes upload as content-addressed blobs. A note references a blob by relative path; the server resolves the path to a hash at push time.

```yaml
# .notesync.yml
attachments:
  max_size: 25MB
  include: ["*.png", "*.jpg", "*.pdf"]
```

Open question: whether to dedupe across trees (same hash, different owners).
