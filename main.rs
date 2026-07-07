use clap::Parser;
use std::process::{exit, Command};
 
/// A tiny echo clone that delegates to the shell's `echo`.
#[derive(Parser, Debug)]
#[command(
    name = "dscribe",
    version,
    about = "A tiny echo clone that shells out to `echo`"
)]
struct Cli {
    /// Text to print
    #[arg(value_name = "TEXT")]
    words: Vec<String>,
 
    /// Do not print the trailing newline
    #[arg(short = 'n')]
    no_newline: bool,
}
 
fn main() {
    let cli = Cli::parse();
 
    // Spawn the actual `echo` program rather than printing in Rust.
    let mut cmd = Command::new("echo");
    //if cli.no_newline {
    //    cmd.arg("-n");
    //}
    //cmd.args(&cli.words);
 
    //match cmd.status() {
    //    Ok(status) => exit(status.code().unwrap_or(0)),
    //    Err(err) => {
    //        eprintln!("dscribe: could not run `echo`: {err}");
            exit(1);
        }
    }
}
 

