#![allow(unused_imports)]

//use std::fs::File;
//use std::io::{self, BufRead, BufReader};

// use clap::{Parser, Subcommand}; 
mod cli;
use crate::cli::*;

mod searcher;
//use crate::searcher::*;

mod tui;
use crate::tui::*;

mod editor;
use crate::editor::*;

use std::path::Path; 
//use std::process::{exit, Command, Stdio};

mod front_matter;
use crate::front_matter::*;

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();
    
    match &cli.command {
        Some (Commands::AddDate) => {
            let _ = tui::enter_tui();
        }
        None => {},
    }
    

    let path = searcher::file_search(cli.dir);

    let tmp_file_name: String = path.clone()
        .as_deref()
        .map(|p| Path::new(p).with_extension("tmp").to_string_lossy().into_owned()).unwrap();
    
    let splitnote = scan_front_matter(path.clone());

    match splitnote {
        NoteState::ContainsFrontMatter {front_matter, body } => {write_front_matter_cache(tmp_file_name, front_matter); rewrite_body(path.clone().unwrap(), body);},
        _ => (),
    }

    editor::launch_editor(get_editor(), path);


    Ok(())
}

