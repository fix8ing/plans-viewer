# plans

A dark, chrome-less reader for a folder of markdown plans. Ships two ways off one UI (`app/index.html`):

```
app/          the page + vendored marked.js — shared by both
plans.mjs     CLI: Node server that serves app/ for a folder
src-tauri/    macOS app: Rust commands replace the server
```

## CLI

```sh
./plans.mjs example/notesync         # serves on http://localhost:4747 and opens it
./plans.mjs ~/dev/foo/plans --port 5000 --no-open
```

Runs on Node ≥ 20 or Bun. Fully offline.

## macOS app

```sh
cargo install tauri-cli --version '^2' --locked   # once
cd src-tauri && cargo tauri build                  # → target/release/bundle/macos/plans.app
```

Open a folder by dropping it on the window, pressing `⌘O`, or from a terminal:
`open -a plans --args ~/dev/foo/plans`. The last folder is remembered.
`cargo tauri dev` runs it against `app/` with a debug build.

## Folder conventions

```
plans/notesync/
├── 00_PRD.md            → "00. PRD"
├── 01_wire/             → "01. Wire" (folder, children nested under it)
│   ├── 00_notes.md      →    "Notes"
│   └── 03_conflicts.md  →    "Conflicts"
└── 05_stacks.md         → "05. Stacks"
```

- Order comes from the `NN_` / `NN-` filename prefix; the number is shown in the sidebar.
- The sidebar label is frontmatter `label`, else the filename with `_`/`-` turned into spaces and its casing kept (`03_conflicts.md` → "Conflicts", `00_PRD.md` → "PRD").
- The page title is frontmatter `title`, else the first `# H1` (which is then lifted out of the body), else the sidebar label.
- The eyebrow above the title is the relative path, then `status`, then every other frontmatter key (except `title`/`label`) as `KEY value`:

  ```yaml
  ---
  status: locked
  rev: "#42"
  ---
  ```

  renders `01_WIRE/03_CONFLICTS · LOCKED · REV #42`.

## Keys

`↑`/`↓` or `j`/`k` move between files. `⌘=` / `⌘-` zoom, `⌘0` resets (remembered). The URL hash holds the open file. Files and the tree re-poll every 1.5 s, so edits show up without a refresh and keep your scroll position.
