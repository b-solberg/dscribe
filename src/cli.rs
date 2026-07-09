pub use clap::{Parser, Subcommand}; 
pub use std::path::PathBuf; 


#[derive(Parser, Debug)]
#[command(
    name = "dscribe",
    version,
    about = "Interactive ripgrep + fzf file search"
)]
pub struct Cli {
    /// Directory to search in (defaults to current directory)
    #[arg(short = 'd', long = "dir", value_name = "DIR")]
    pub dir: Option<PathBuf>,
    
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    AddDate
}