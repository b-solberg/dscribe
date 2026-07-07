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
    /// Text to echo (if provided, behaves like echo instead of searching)
    #[arg(value_name = "TEXT")]
    words: Vec<String>,
 
    /// Do not print the trailing newline (echo mode only)
    #[arg(short = 'n')]
    no_newline: bool,
 
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
            Ok(if sel.is_empty() { None } else { Some(sel) })
        }
        _ => Ok(None),
    }
}
 
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
 
    // If words were passed, behave like echo
    if !cli.words.is_empty() {
        let mut cmd = Command::new("echo");
        if cli.no_newline {
            cmd.arg("-n");
        }
        cmd.args(&cli.words);
        match cmd.status() {
            Ok(status) => exit(status.code().unwrap_or(0)),
            Err(err) => {
                eprintln!("dscribe: could not run `echo`: {err}");
                exit(1);
            }
        }
    }
 
    // Otherwise run the interactive finder
    match interactive_grep(cli.dir)? {
        Some(line) => {
            // line == "path:lineno:matched text"
            let mut it = line.splitn(3, ':');
            let path = it.next().unwrap_or("");
            let lineno = it.next().unwrap_or("");
            println!("{path}:{lineno}");
        }
        None => eprintln!("nothing selected"),
    }
 
    Ok(())
}

