---
status: draft
label: Pre-reqs
---

# Pre-reqs

- OpenAPI spec published from the server repo on every merge.
- A `PUT /tree` endpoint that accepts a full manifest (today it's per-note).
- Hash function agreed: BLAKE3 over raw bytes, hex-encoded.
