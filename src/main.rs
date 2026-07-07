use std::process::{Command, Stdio};

/// Runs `rg . | fzf` (with a bat preview) in the current directory and
/// returns the line the user selected, or None if they picked nothing.
fn interactive_grep() -> std::io::Result<Option<String>> {
    let mut rg = Command::new("rg")
        .args(["--line-number", "--no-heading", "--color", "never", "."])
        .stdout(Stdio::piped())
        .spawn()?;

    let rg_out = rg.stdout.take().expect("rg stdout was piped");

    let fzf = Command::new("fzf")
        .args([
            "--delimiter", ":",
            "--preview", "bat --style=numbers --color=always --highlight-line {2} {1}",
            "--preview-window", "right:60%",
        ])
        .stdin(Stdio::from(rg_out))   // rg's output feeds fzf's stdin
        .stdout(Stdio::piped())        // capture the selection
        .spawn()?;                     // stderr inherited; fzf's UI is on /dev/tty

    let out = fzf.wait_with_output()?;
    let _ = rg.wait(); // reap rg (it gets SIGPIPE if fzf exits early)

    match out.status.code() {
        Some(0) => {
            let sel = String::from_utf8_lossy(&out.stdout).trim_end().to_string();
            Ok(if sel.is_empty() { None } else { Some(sel) })
        }
        _ => Ok(None), // 1 = no match, 130 = aborted
    }
}

fn main() -> std::io::Result<()> {
    match interactive_grep()? {
        Some(line) => {
            // line == "path:lineno:matched text"
            let mut it = line.splitn(3, ':');
            let path = it.next().unwrap_or("");
            let lineno = it.next().unwrap_or("");
            println!("{path} @ {lineno}");
        }
        None => eprintln!("nothing selected"),
    }
    Ok(())
}
