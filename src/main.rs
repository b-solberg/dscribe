#![allow(unused_imports)]
// use clap::{Parser, Subcommand}; 
mod cli;
use crate::cli::*;

mod searcher;
//use crate::searcher::*;

mod tui;
use crate::tui::*;

mod editor;
use crate::editor::*;

//use std::path::PathBuf; 
//use std::process::{exit, Command, Stdio};

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();
    
    match &cli.command {
        Some (Commands::AddDate) => {
            //println!("TODO Add Date function/TUI");
            tui::enter_tui();
        }
        None => {},
    }
 
    let path = searcher::file_search(cli.dir);
    
    editor::launch_editor(get_editor(), path);
    Ok(())
}

