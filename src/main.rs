mod cli;
use crate::cli::*;

mod searcher;

mod tui;

mod editor;
use crate::editor::*;

use anyhow::{ Result};

mod front_matter;
use crate::front_matter::*;

use anyhow::anyhow;
use time::format_description;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let mut added_date = String::new();
    let format = format_description::parse_borrowed::<3>("[month]-[day]-[year]")?;
    if cli.add_date {
            match tui::select_date() {
                Ok(Some(date)) => added_date = format!{"{}{}", added_date, date.format(&format)?},
                Ok(None) => {},
                Err(_) => {},
            }
    }
    println!("{}", added_date);

    let path = match searcher::file_search(cli.dir){
        Ok(path) => path,
        Err(e) => return Err(anyhow!("{}", e))



    };

    let splitnote = scan_front_matter(path.clone());
    if cli.remove_front_matter {
        todo!();
    }
    match splitnote? {
        NoteState::ContainsFrontMatter {front_matter, body} => {write_front_matter_cache(path.clone(), &front_matter); rewrite_body(path.clone(),&body)}, 
        _ => (),
    }
    
    editor::launch_editor(get_editor(), path.clone());

    let original_front_matter = get_front_matter_cache(path.clone());
    join_front_matter_and_body(path.clone(), original_front_matter)?;

    clear_front_matter_cache(path.clone());
    
    Ok(())
}

