---
status: draft
---

# Execution

Two people, three weeks. Server work lands first so the client can be tested against it.

1. Server: `PUT /tree` + conflict rule.
2. Client: manifest + push, generated from the spec.
3. Client: pull, then the `.conflict.md` round-trip.
4. Docs and the CI recipe.
