pub use clap::{Parser}; 
pub use std::path::PathBuf; 


#[derive(Parser, Debug)]
#[command(
    name = "dscribe",
    version,
    about = "Interactive ripgrep + fzf file search"
)]
pub struct Cli {
    /// Directory to search in (defaults to current directory)
    pub dir: Option<PathBuf>,

    #[arg(short = 'a', long = "add_date")]
    pub add_date: bool,

    #[arg(short = 'f', long = "remove_front_matter")]
    pub remove_front_matter: bool,

    //#[command(subcommand)]
    //pub command: Option<Commands>,
}

//#[derive(Subcommand,Debug)]
//pub enum Commands {
//    AddDate
//}
