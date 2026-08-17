use dirs::cache_dir;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use anyhow::{Context, Result};
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub enum NoteState {
    ContainsFrontMatter {front_matter: Vec<String>, body: Vec<String>},
    NoFrontMatter {body: Vec<String>},
}
pub fn write_file(file: String, contents: &Vec<String>) {
    let file = File::create(file).unwrap();

    let mut writer = BufWriter::new(file);

    for line in contents {
        writeln!(writer, "{}", line).unwrap();
    }

    writer.flush().unwrap();
}

fn cache_location() -> PathBuf {
    let base = cache_dir().unwrap_or_else(std::env::temp_dir);
    let dir = base.join("dscribe");
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn get_front_matter_cache(file_name: String) -> Result<Vec<String>> {
    let tmp_file = tmp_file_extension(file_name);
    let dir = cache_location();
    let dir = dir.join(tmp_file);
    let file = File::open(dir.to_string_lossy().into_owned())?;
    let reader = BufReader::new(file);
    let mut collected_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        collected_lines.push(line?)
    }
    Ok(collected_lines)
}

pub fn clear_front_matter_cache(file_name:String) {
    let tmp_file = tmp_file_extension(file_name);
    let dir = cache_location();
    let dir = dir.join(tmp_file);
    match std::fs::remove_file(dir.to_string_lossy().into_owned()) {
        Ok(_s) => (),
        Err(_e) => (),

    };
}

pub fn write_front_matter_cache(file_name: String, front_matter: &Vec<String>) {
    let tmp_file = tmp_file_extension(file_name);
    let dir = cache_location();
    let dir = dir.join(tmp_file);
    //println!("{:?}",dir);
    write_file(dir.to_string_lossy().into_owned(), front_matter);
}

pub fn rewrite_body(file: String, body: &Vec<String>) {
    let file_loc = File::create(file).unwrap();
    let mut writer = BufWriter::new(file_loc);

    for line in body {
        writeln!(writer, "{}", line).unwrap();
    }
    writer.flush().unwrap();
}

pub fn join_front_matter_and_body(file: String, front_matter_original: Result<Vec<String>>) -> Result<()> {
    match front_matter_original {
        Ok(mut front_matter_original) => {
            let state_post_edit = scan_front_matter(file.clone());
            let mut new_front_matter: Vec<String> = Vec::new();
            new_front_matter.push("---".to_string());

            match state_post_edit? {
                NoteState::ContainsFrontMatter {mut front_matter, mut body} => {
                    new_front_matter.append(&mut front_matter_original);
                    new_front_matter.append(&mut front_matter);
                    new_front_matter.push("---".to_string());
                    new_front_matter.append(&mut body);
                    //println!("{:?}", new_front_matter);
                    write_file(file, &new_front_matter);
                },
                NoteState::NoFrontMatter {mut body} => {
                    new_front_matter.append(&mut front_matter_original);
                    new_front_matter.push("---".to_string());
                    new_front_matter.append(&mut body);
                    write_file(file, &new_front_matter);
                }, 
            }
        },
    Err(_e) => ()
    };
    Ok(())
}

pub fn tmp_file_extension(file: String) -> String {
    let file_name = Path::new(&file)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    Path::new(&file_name).with_extension("tmp").to_string_lossy().into_owned()

}

//I think this is done for now
pub fn scan_front_matter(file_name: String) -> Result<NoteState> {
    let file = File::open(file_name).context("Failed to Open File After Selection")?;
    let reader = BufReader::new(file);
    let mut state = Scan::Seeking; 
    let mut collected_lines: Vec<String> = Vec::new();
    for (line_number, line) in reader.lines().enumerate() {

        let line = line.unwrap(); // each item is io::Result<String>
        collected_lines.push(line.clone());
        let test_delimiter = match line.trim_end() {
            "---" => true,
            _ => false 
        };
        state = state.step_by_line(test_delimiter, line_number);
        
    }
    Ok(match state {
        Scan::Absent | Scan::PotentiallyFrontMatter | Scan::Seeking => {
            //println!("{:?}", collected_lines);
            NoteState::NoFrontMatter {body:collected_lines}},
        Scan::ExitFrontMatter { end } => {
            let body = collected_lines.split_off(end);
            //println!("{:?}", body);
            collected_lines.pop(); collected_lines.remove(0);
            //println!("{:?}", collected_lines);
            NoteState::ContainsFrontMatter { front_matter: collected_lines, body: body}
        }
    })
    
}

#[derive(Debug)]
pub enum Scan {
    Seeking,
    PotentiallyFrontMatter,
    ExitFrontMatter {end:usize}, 
    Absent,
}

impl Scan {
    pub fn step_by_line(self, is_delimiter: bool, line_number:usize) -> Scan {
        match self {
            Scan::Seeking if is_delimiter => Scan::PotentiallyFrontMatter,
            Scan::Seeking => Scan::Absent,
            Scan::PotentiallyFrontMatter if is_delimiter => Scan::ExitFrontMatter { end:line_number+1 },
            Scan::PotentiallyFrontMatter if !is_delimiter => Scan::PotentiallyFrontMatter,
            _ => self
        }
    }
}



//pub fn join_front_matters(front_matters: Vec<String>) -> String {
//    
//
//
//}
