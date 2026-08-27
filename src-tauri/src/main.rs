#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plans;

use std::path::{Path, PathBuf};

use plans::{Doc, Tree};

#[tauri::command]
fn tree(root: String) -> Result<Tree, String> {
    plans::tree(Path::new(&root)).map_err(|e| e.to_string())
}

#[tauri::command]
fn doc(root: String, path: String) -> Result<Doc, String> {
    plans::doc(Path::new(&root), &path).map_err(|e| e.to_string())
}

/// The folder passed on the command line, if any.
#[tauri::command]
fn initial_root() -> Option<String> {
    let arg = PathBuf::from(std::env::args().nth(1)?);
    let dir = if arg.is_file() { arg.parent()?.to_path_buf() } else { arg };
    dir.canonicalize().ok()?.to_str().map(String::from)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![tree, doc, initial_root])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
