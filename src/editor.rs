pub use std::process::{Command};

pub fn get_editor() -> String {
    std::env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {"nvim".into() } else {"notepad".into() }
    })
}

pub fn launch_editor(editor: String, file_path:String) {
        Command::new(editor).arg(&file_path).status().expect("Failed to Launch Editor on File");

}
