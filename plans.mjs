#!/usr/bin/env node
// plans — serve a folder of markdown plans as a dark reading UI.
//
//   plans [dir] [--port N] [--no-open]
//
// Files are ordered by their `NN_` / `NN-` prefix; folders nest one level deeper.
// Frontmatter `label:` names the sidebar entry; `status:` and any other scalar key
// show in the eyebrow above the title.

import { readdir, readFile, stat } from "node:fs/promises";
import { createServer } from "node:http";
import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const flag = (name) => {
  const i = args.indexOf(name);
  return i === -1 ? null : args.splice(i, 2)[1];
};
const noOpen = args.includes("--no-open");
if (noOpen) args.splice(args.indexOf("--no-open"), 1);
const port = Number(flag("--port") ?? 4747);
const root = path.resolve(args[0] ?? ".");
const rootLabel = path.relative(process.cwd(), root) || path.basename(root);

const APP = new URL("./app/", import.meta.url);
const MIME = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css" };

const PREFIX = /^(\d+)[_\-. ]+/;
const humanize = (slug) => {
  const s = slug.replace(PREFIX, "").replace(/[-_]+/g, " ").trim();
  return s.charAt(0).toUpperCase() + s.slice(1);
};

function parseFrontmatter(src) {
  const m = src.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n?/);
  if (!m) return { fm: {}, body: src };
  const fm = {};
  for (const line of m[1].split(/\r?\n/)) {
    const kv = line.match(/^([\w-]+):\s*(.*)$/);
    if (kv) fm[kv[1]] = kv[2].replace(/^["']|["']$/g, "");
  }
  return { fm, body: src.slice(m[0].length) };
}

function splitTitle(fm, body, fallback) {
  const h1 = body.match(/^#\s+(.+?)\s*$/m);
  const title = fm.title ?? h1?.[1] ?? fallback;
  if (!fm.title && h1) body = body.replace(h1[0], "");
  return { title, body: body.trim() };
}

async function readDoc(rel) {
  const abs = path.join(root, rel);
  const [src, st] = await Promise.all([readFile(abs, "utf8"), stat(abs)]);
  const { fm, body: raw } = parseFrontmatter(src);
  const { title, body } = splitTitle(fm, raw, humanize(path.basename(rel, ".md")));
  return { path: rel, title, body, fm, mtime: st.mtimeMs };
}

async function walk(rel = "") {
  const entries = await readdir(path.join(root, rel), { withFileTypes: true });
  const nodes = [];
  for (const e of entries.sort((a, b) => a.name.localeCompare(b.name))) {
    if (e.name.startsWith(".")) continue;
    const childRel = path.join(rel, e.name);
    const number = e.name.match(PREFIX)?.[1] ?? null;
    if (e.isDirectory()) {
      const children = await walk(childRel);
      if (children.length) nodes.push({ kind: "dir", number, title: humanize(e.name), children });
    } else if (e.name.endsWith(".md")) {
      const { fm } = parseFrontmatter(await readFile(path.join(root, childRel), "utf8"));
      const title = fm.label ?? humanize(e.name.slice(0, -3));
      nodes.push({ kind: "file", number, title, path: childRel });
    }
  }
  return nodes;
}

function json(res, status, value) {
  res.writeHead(status, { "content-type": "application/json" });
  res.end(JSON.stringify(value));
}

const server = createServer(async (req, res) => {
  const url = new URL(req.url, "http://localhost");
  try {
    if (!url.pathname.startsWith("/api/")) {
      const file = url.pathname === "/" ? "index.html" : path.basename(url.pathname);
      const body = await readFile(new URL(file, APP));
      res.writeHead(200, { "content-type": MIME[path.extname(file)] ?? "application/octet-stream" });
      return res.end(body);
    }
    if (url.pathname === "/api/tree") return json(res, 200, { root: rootLabel, tree: await walk() });
    if (url.pathname === "/api/doc") {
      const rel = path.normalize(url.searchParams.get("path") ?? "");
      if (rel.startsWith("..") || !rel.endsWith(".md")) return json(res, 400, { error: "bad path" });
      return json(res, 200, await readDoc(rel));
    }
    json(res, 404, { error: "not found" });
  } catch (err) {
    json(res, err.code === "ENOENT" ? 404 : 500, { error: err.message });
  }
});

server.listen(port, "127.0.0.1", () => {
  const addr = `http://localhost:${port}`;
  console.log(`plans: ${rootLabel} → ${addr}`);
  if (!noOpen && process.platform === "darwin") spawn("open", [addr], { stdio: "ignore" });
});
