#![allow(unused_imports)]

pub use std::path::PathBuf; 
pub use std::process::{exit, Command, Stdio};

pub fn get_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {"nvim".into() } else {"notepad".into() }
    })
}

pub fn launch_editor(editor: String, path:Option<String>) {
    match path {
        Some(file_path) => {Command::new(editor).arg(&file_path).status().expect("Failed to Launch Editor");},
        None => {},
    };
}
