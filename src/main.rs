#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    //let cli = cli::Cli::parse();
    //
    //match &cli.command {
    //    Some (Commands::AddDate) => {
    //        let _ = tui::enter_tui();
    //    }
    //    None => {},
    //}
    let path = File::open("front_matter_example.md")?;
    let reader = BufReader::new(path);
    let mut state = editor::Scan::Seeking;
    for (line_number, line) in reader.lines().enumerate() {

        let line = line?; // each item is io::Result<String>
        let test_delimiter = match line.trim_end() {
            "---" => true,
            _ => false 
        };
        state = state.step_by_line(test_delimiter, line_number);
        match state {
            editor::Scan::Absent | editor::Scan::ExitFrontMatter {start: _, end: _} => {println!("Detected Front Matter"); break},
            _ => continue
        }
    } 


    //let path = searcher::file_search(cli.dir);
    

    //editor::launch_editor(get_editor(), path);
    Ok(())
}

