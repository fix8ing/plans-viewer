---
status: locked
rev: "#42"
---

# Conflicts

The server compares each note's `base_hash` with the current remote hash. Three outcomes, no prompt.

| base_hash vs remote | Result |
|---|---|
| equal | write accepted |
| `null`, note exists remotely | rejected — pull first |
| differs | remote copy kept as `<name>.conflict.md`, local write accepted |

### Decisions

- **Local always wins** on a real conflict. The user asked for a push; the conflict file preserves what they'd otherwise lose.
- `.conflict.md` files are ordinary notes. They sync like any other and are deleted like any other.
