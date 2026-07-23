#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use dirs::cache_dir;

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub enum NoteState {
    ContainsFrontMatter {front_matter: Vec<String>, body: Vec<String>},
    NoFrontMatter,
}
pub fn write_file(file: String, contents: Vec<String>) {
    let file = File::create(file).unwrap();

    let mut writer = BufWriter::new(file);

    for line in contents {
        writeln!(writer, "{}", line).unwrap();
    }

    writer.flush().unwrap();
}
pub fn write_front_matter_cache(file: String, front_matter: Vec<String>) {
    let cache_file_loc = cache_location(file);
    println!("{:?}", cache_file_loc);
    write_file(cache_file_loc.to_string_lossy().into_owned(), front_matter);
}

pub fn cache_location(file: String) -> PathBuf {
    let base = cache_dir().unwrap_or_else(std::env::temp_dir);
    let dir = base.join("dscribe");
    std::fs::create_dir_all(&dir).ok();
    dir.join(file)
}

pub fn rewrite_body(file: String, body: Vec<String>) {
    let file_loc = File::create(file).unwrap();
    let mut writer = BufWriter::new(file_loc);

    for line in body {
        writeln!(writer, "{}", line).unwrap();
    }
    writer.flush().unwrap();
}

pub fn join_front_matter_and_body(file: String) {
    todo!()
}

pub fn tmp_file_extension(file: String) -> String {
    Path::new(&file).with_extension("tmp").to_string_lossy().into_owned()

}

//I think this is done for now
pub fn scan_front_matter(file: Option<String>) -> NoteState {
    let path = File::open(file.unwrap()).unwrap();
    let reader = BufReader::new(path);
    let mut state = Scan::Seeking; 
    let mut collected_lines: Vec<String> = Vec::new();
    for (line_number, line) in reader.lines().enumerate() {

        let line = line.unwrap(); // each item is io::Result<String>
        collected_lines.push(line.clone());
        let test_delimiter = match line.trim_end() {
            "---" => true,
            _ => false 
        };
        state = state.step_by_line(test_delimiter, line_number, &line);
        match state {
            Scan::Absent => break,
            _ => continue
        }

    }
    match state {
        Scan::Absent | Scan::PotentiallyFrontMatter {start: _ }| Scan::Seeking => NoteState::NoFrontMatter,
        Scan::ExitFrontMatter {start , end } => NoteState::ContainsFrontMatter { front_matter: collected_lines[start..=end].to_vec(), body: collected_lines[end+1..].to_vec()}
    }
    
}

#[derive(Debug)]
pub enum Scan {
    Seeking,
    PotentiallyFrontMatter {start: usize},//, front_matter: Vec<String>},
    ExitFrontMatter {start: usize, end:usize}, //front_matter: Vec<String>},
    Absent,
}

impl Scan {
    pub fn step_by_line(self, is_delimiter: bool, line_number:usize, line: &String) -> Scan {
        match self {
            Scan::Seeking if is_delimiter => Scan::PotentiallyFrontMatter{ start: line_number },
            
            Scan::Seeking => Scan::Absent,
            Scan::PotentiallyFrontMatter { start } if is_delimiter => {
                //front_matter.push(line); 
                Scan::ExitFrontMatter {start, end:line_number }},
            Scan::PotentiallyFrontMatter { start } if !is_delimiter => {
                //front_matter.push(line); 
                Scan::PotentiallyFrontMatter {start }},
            _ => self
        }
    }
}
