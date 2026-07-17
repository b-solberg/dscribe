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


#[derive(Debug)]
pub enum Scan {
    Seeking,
    PotentiallyFrontMatter {start: usize},
    ExitFrontMatter {start: usize, end:usize},
    Absent,
}

impl Scan {
    pub fn step_by_line(self, is_delimiter: bool, line_number:usize) -> Scan {
        match self {
            Scan::Seeking if is_delimiter => Scan::PotentiallyFrontMatter{ start: line_number },
            Scan::Seeking => Scan::Absent,
            Scan::PotentiallyFrontMatter { start } if is_delimiter => Scan::ExitFrontMatter {start, end:line_number},
            _ => self
        }
    }
}


