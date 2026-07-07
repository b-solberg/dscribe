use clap::Parser;
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
 
#[derive(Parser, Debug)]
#[command(
    name = "dscribe",
    version,
    about = "Interactive ripgrep + fzf file search, with echo fallback"
)]
struct Cli {
    /// Directory to search in (defaults to current directory)
    #[arg(short = 'd', long = "dir", value_name = "DIR")]
    dir: Option<PathBuf>,
}
 
fn interactive_grep(dir: Option<PathBuf>) -> std::io::Result<Option<String>> {
    let search_dir = match dir {
        Some(ref d) => d.clone(),
        None => std::env::current_dir()?,
    };
 
    let mut rg = Command::new("rg")
        .args(["--line-number", "--no-heading", "--color", "never", "."])
        .current_dir(&search_dir)
        .stdout(Stdio::piped())
        .spawn()?;
 
    let rg_out = rg.stdout.take().expect("rg stdout was piped");
 
    let fzf = Command::new("fzf")
        .args([
            "--delimiter", ":",
            "--preview", "bat --style=numbers --color=always --highlight-line {2} {1}",
            "--preview-window", "right:60%",
        ])
        .stdin(Stdio::from(rg_out))
        .stdout(Stdio::piped())
        .current_dir(&search_dir)
        .spawn()?;
 
    let out = fzf.wait_with_output()?;
    let _ = rg.wait();
 
    match out.status.code() {
        Some(0) => {

            let sel = String::from_utf8_lossy(&out.stdout).trim_end().to_string();
            Ok(if sel.is_empty() { None } else {
                let mut it = sel.splitn(3, ':');
                let file_name = it.next().unwrap_or("").to_string();
                let full_path = PathBuf::from(&search_dir).join(&file_name);
                Some(full_path.to_string_lossy().into_owned()) })
        }
        _ => Ok(None),
    }
}

//fn open_editor

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
 
    let path: String = match interactive_grep(cli.dir)? {
        Some(line) => { 
            //let mut it = line.splitn(3, ':');
            //let path = it.next().unwrap_or("").to_string();
            //println!("{line}");
            line
        }
        None => {
            eprintln!("Exited without selection");
            String::new()    
        },
    };
    
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {"nvim".into() } else {"notepad".into() }
    });
    //println!("{editor}");

   Command::new(editor).arg(&path).status().expect("failed to launch editor"); 

    Ok(())

}

