//! Reads a folder of markdown plans into the tree and document shapes the page renders.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Component, Path};
use std::time::UNIX_EPOCH;

use serde::Serialize;

/// One sidebar entry.
#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Node {
    Dir {
        number: Option<String>,
        title: String,
        children: Vec<Node>,
    },
    File {
        number: Option<String>,
        title: String,
        path: String,
    },
}

/// The sidebar: a label for the folder and its entries.
#[derive(Debug, Serialize)]
pub struct Tree {
    pub root: String,
    pub tree: Vec<Node>,
}

/// One rendered document.
#[derive(Debug, Serialize)]
pub struct Doc {
    pub path: String,
    pub title: String,
    pub body: String,
    pub fm: BTreeMap<String, String>,
    pub mtime: f64,
}

pub fn tree(root: &Path) -> io::Result<Tree> {
    let label = root
        .components()
        .rev()
        .take(2)
        .filter_map(|c| c.as_os_str().to_str())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/");
    Ok(Tree { root: label, tree: walk(root, "")? })
}

pub fn doc(root: &Path, rel: &str) -> io::Result<Doc> {
    let safe = Path::new(rel)
        .components()
        .all(|c| matches!(c, Component::Normal(_)));
    if !safe || !rel.ends_with(".md") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad path"));
    }
    let abs = root.join(rel);
    let src = fs::read_to_string(&abs)?;
    let mtime = fs::metadata(&abs)?
        .modified()?
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0);
    let (fm, raw) = parse_frontmatter(&src);
    let stem = Path::new(rel).file_stem().and_then(|s| s.to_str()).unwrap_or(rel);
    let fallback = fm.get("label").cloned().unwrap_or_else(|| humanize(stem));
    let (title, body) = split_title(&fm, raw, fallback);
    Ok(Doc { path: rel.to_string(), title, body, fm, mtime })
}

fn walk(root: &Path, rel: &str) -> io::Result<Vec<Node>> {
    let mut entries: Vec<_> = fs::read_dir(root.join(rel))?
        .filter_map(Result::ok)
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut nodes = Vec::new();
    for e in entries {
        let name = e.file_name().to_string_lossy().into_owned();
        let child_rel = if rel.is_empty() { name.clone() } else { format!("{rel}/{name}") };
        let number = number_prefix(&name);
        if e.file_type()?.is_dir() {
            let children = walk(root, &child_rel)?;
            if !children.is_empty() {
                nodes.push(Node::Dir { number, title: humanize(&name), children });
            }
        } else if let Some(stem) = name.strip_suffix(".md") {
            let (fm, _) = parse_frontmatter(&fs::read_to_string(e.path())?);
            let title = fm.get("label").cloned().unwrap_or_else(|| humanize(stem));
            nodes.push(Node::File { number, title, path: child_rel });
        }
    }
    Ok(nodes)
}

/// Splits `NN_rest` / `NN-rest` / `NN. rest` into the number and the rest.
fn split_prefix(name: &str) -> Option<(&str, &str)> {
    let digits = name.len() - name.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 {
        return None;
    }
    let rest = name[digits..].trim_start_matches(['_', '-', '.', ' ']);
    if rest.len() == name.len() - digits {
        return None;
    }
    Some((&name[..digits], rest))
}

fn number_prefix(name: &str) -> Option<String> {
    split_prefix(name).map(|(n, _)| n.to_string())
}

fn humanize(slug: &str) -> String {
    let rest = split_prefix(slug).map(|(_, r)| r).unwrap_or(slug);
    let spaced = rest.replace(['-', '_'], " ");
    let spaced = spaced.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn parse_frontmatter(src: &str) -> (BTreeMap<String, String>, &str) {
    let mut fm = BTreeMap::new();
    let Some(after_open) = src.strip_prefix("---\n").or_else(|| src.strip_prefix("---\r\n")) else {
        return (fm, src);
    };
    let Some(end) = after_open.find("\n---") else {
        return (fm, src);
    };
    for line in after_open[..end].lines() {
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim();
            if !k.is_empty() && k.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                fm.insert(k.to_string(), v.trim().trim_matches(['"', '\'']).to_string());
            }
        }
    }
    let body = &after_open[end + 4..];
    (fm, body.strip_prefix('\n').or_else(|| body.strip_prefix("\r\n")).unwrap_or(body))
}

fn split_title(fm: &BTreeMap<String, String>, body: &str, fallback: String) -> (String, String) {
    let h1 = body.lines().find_map(|l| {
        l.strip_prefix("# ").map(|t| (l, t.trim().to_string()))
    });
    if let Some(title) = fm.get("title") {
        return (title.clone(), body.trim().to_string());
    }
    match h1 {
        Some((line, title)) => (title, body.replacen(line, "", 1).trim().to_string()),
        None => (fallback, body.trim().to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanize_keeps_casing_and_strips_prefix() {
        assert_eq!(humanize("00_PRD"), "PRD");
        assert_eq!(humanize("08_stripe-mappings"), "Stripe mappings");
        assert_eq!(humanize("09_RevenueCat-mappings"), "RevenueCat mappings");
        assert_eq!(humanize("wire"), "Wire");
    }

    #[test]
    fn number_prefix_needs_a_separator_and_a_rest() {
        assert_eq!(number_prefix("06_stacks"), Some("06".into()));
        assert_eq!(number_prefix("2024"), None);
        assert_eq!(number_prefix("stacks"), None);
    }

    #[test]
    fn frontmatter_is_split_and_unquoted() {
        let (fm, body) = parse_frontmatter("---\nstatus: locked\nrev: \"#3093\"\n---\n\n# T\n\nbody\n");
        assert_eq!(fm["status"], "locked");
        assert_eq!(fm["rev"], "#3093");
        assert_eq!(body, "\n# T\n\nbody\n");
    }

    #[test]
    fn h1_becomes_the_title_and_leaves_the_body() {
        let (title, body) = split_title(&BTreeMap::new(), "# PR stacks\n\nThree stacks.", "Stacks".into());
        assert_eq!(title, "PR stacks");
        assert_eq!(body, "Three stacks.");
    }

    #[test]
    fn doc_rejects_escaping_paths() {
        assert!(doc(Path::new("."), "../x.md").is_err());
        assert!(doc(Path::new("."), "x.txt").is_err());
    }
}
