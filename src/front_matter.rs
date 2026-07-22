#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use dirs::cache_dir;

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn write_front_matter_cache(file: &str, front_matter: Vec<String>) {
    let cache_file_loc = cache_location(file);
    println!("{:?}", cache_file_loc);
    let cache_file = File::create(cache_file_loc).unwrap();
    let mut writer = BufWriter::new(cache_file);

    for line in front_matter {
        writeln!(writer, "{}", line).unwrap();
    }
    writer.flush().unwrap();
}

pub fn cache_location(file: &str) -> PathBuf {
    let base = cache_dir().unwrap_or_else(std::env::temp_dir);
    let dir = base.join("dscribe");
    std::fs::create_dir_all(&dir).ok();
    dir.join(file)
}

pub fn scan_front_matter(file: &str ) -> Scan {
    let path = File::open(file).unwrap();
    let reader = BufReader::new(path);
    let mut state = Scan::Seeking;
    for (line_number, line) in reader.lines().enumerate() {

        let line = line.unwrap(); // each item is io::Result<String>
        let test_delimiter = match line.trim_end() {
            "---" => true,
            _ => false 
        };
        state = state.step_by_line(test_delimiter, line_number, line);
        match state {
            Scan::Absent => break,
            Scan::ExitFrontMatter {start: _, end: _, front_matter: _ } => {println!("Detected Front Matter"); break},
            _ => continue
        }
    }
    state
}

#[derive(Debug)]
pub enum Scan {
    Seeking,
    PotentiallyFrontMatter {start: usize, front_matter: Vec<String>},
    ExitFrontMatter {start: usize, end:usize, front_matter: Vec<String>},
    Absent,
}

impl Scan {
    pub fn step_by_line(self, is_delimiter: bool, line_number:usize, line: String) -> Scan {
        match self {
            Scan::Seeking if is_delimiter => Scan::PotentiallyFrontMatter{ start: line_number , front_matter: vec![line] },
            
            Scan::Seeking => Scan::Absent,
            Scan::PotentiallyFrontMatter { start, mut front_matter } if is_delimiter => {
                front_matter.push(line); 
                Scan::ExitFrontMatter {start, end:line_number, front_matter}},
            Scan::PotentiallyFrontMatter { start, mut front_matter } if !is_delimiter => {
                front_matter.push(line); 
                Scan::PotentiallyFrontMatter {start, front_matter}},
            _ => self
        }
    }
}
