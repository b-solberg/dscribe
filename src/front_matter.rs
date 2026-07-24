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
pub fn write_file(file: String, contents: &Vec<String>) {
    let file = File::create(file).unwrap();

    let mut writer = BufWriter::new(file);

    for line in contents {
        writeln!(writer, "{}", line).unwrap();
    }

    writer.flush().unwrap();
}
pub fn write_front_matter_cache(file: Option<String>, front_matter: &Vec<String>) {
    let file_name = Path::new(&file.unwrap())
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let tmp_file = tmp_file_extension(file_name);
    let base = cache_dir().unwrap_or_else(std::env::temp_dir);
    let dir = base.join("dscribe");
    std::fs::create_dir_all(&dir).ok();
    let dir = dir.join(tmp_file);
    println!("{:?}",dir);
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

pub fn join_front_matter_and_body(file: String, front_matter: &Vec<String>) {
   todo!(); 
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
        state = state.step_by_line(test_delimiter, line_number);
        match state {
            Scan::Absent => break,
            _ => continue
        }

    }
    match state {
        Scan::Absent | Scan::PotentiallyFrontMatter | Scan::Seeking => NoteState::NoFrontMatter,
        Scan::ExitFrontMatter { end } => {
            let body = collected_lines.split_off(end);
            NoteState::ContainsFrontMatter { front_matter: collected_lines, body: body}
        }
    }
    
}

#[derive(Debug)]
pub enum Scan {
    Seeking,
    PotentiallyFrontMatter,// {start: usize},//, front_matter: Vec<String>},
    ExitFrontMatter {end:usize}, //front_matter: Vec<String>},
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
