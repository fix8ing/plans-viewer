---
status: locked
---

# Tags

Tags are not sent. The server derives them from `#hashtags` and front-matter `tags:` in each note body, so they can never drift from content.

### Decisions

- Tag rename is a search-and-replace on the client, then a normal push.
- Tags are case-insensitive; the server keeps the first spelling it saw.
