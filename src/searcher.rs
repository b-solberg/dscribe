extern crate skim;
use skim::prelude::*;
use std::path::{Path, PathBuf};
use std::io::ErrorKind;
use ignore::Walk;

use anyhow::{Context, Result, anyhow};
//use std::process::{exit, Command, Stdio};
use std::sync::Arc;



struct MultiLineItem {
    body: String,
    oneline: String,
}

impl MultiLineItem {
    fn new(body:String) -> Self {
        let oneline = body.replace('\n', "  ");
        Self {body , oneline}
    }
}

impl SkimItem for MultiLineItem {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.oneline)
    }
    fn preview(&self, _context: PreviewContext<'_>) -> ItemPreview {
        ItemPreview::Text(self.body.clone())

    }

    fn output(&self) ->  Cow<'_, str> {
        Cow::Borrowed(&self.body)
    }
}

//pub fn interactive_grep(dir: Option<PathBuf>) -> std::io::Result<Option<String>> {
//    let search_dir = match dir {
//        Some(ref d) => d.clone(),
//        None => std::env::current_dir()?,
//    };
// 
//    let mut rg = Command::new("rg")
//        .args(["--line-number", "--no-heading", "--color", "never", "."])
//        .current_dir(&search_dir)
//        .stdout(Stdio::piped())
//        .spawn()?;
// 
//    let rg_out = rg.stdout.take().expect("rg stdout was piped");
// 
//    let fzf = Command::new("fzf")
//        .args([
//            "--delimiter", ":",
//            "--preview", "bat --style=numbers --color=always --highlight-line {2} {1}",
//            "--preview-window", "right:60%",
//        ])
//        .stdin(Stdio::from(rg_out))
//        .stdout(Stdio::piped())
//        .current_dir(&search_dir)
//        .spawn()?;
// 
//    let out = fzf.wait_with_output()?;
//    let _ = rg.wait();
// 
//    match out.status.code() {
//        Some(0) => {
//
//            let sel = String::from_utf8_lossy(&out.stdout).trim_end().to_string();
//            Ok(if sel.is_empty() { None } else {
//                let mut it = sel.splitn(3, ':');
//                let file_name = it.next().unwrap_or("").to_string();
//                let full_path = PathBuf::from(&search_dir).join(&file_name);
//                Some(full_path.to_string_lossy().into_owned()) })
//        }
//        _ => Ok(None),
//    }
//}

pub fn file_search(dir: Option<PathBuf>) -> Result<String> {
    let search_dir = match dir {
        Some(ref d) => d.clone(),
        None => std::env::current_dir()?,
    };
    
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    let mut items: Vec<MultiLineItem> = Vec::new();

    for result in Walk::new(search_dir) {
        match result {
            Ok(entry) => if entry.path().is_dir() {
                    continue;
                }
                else {
                    match read_file(entry.path()){
                    Ok(Some(contents)) => items.push(MultiLineItem::new(format!("{}:{}", entry.path().display(),contents))),
                    Ok(None) => continue,
                    Err(e) => return Err(e)
                }
            },
                
            Err(err) => println!("ERROR: {}", err),
        }
    }  

    let skimitems: Vec<Arc<dyn SkimItem>> = items.
        into_iter()
        .map(|item| Arc::new(item) as Arc<dyn SkimItem>)
        .collect();
    tx.send(skimitems)?;
    drop(tx);
    
    
    let options = SkimOptionsBuilder::default()
        //.delimiter(Regex::new(":").expect("invalid regex delimiter"))
        .preview("echo {}")
        .preview_window("right:60%")
        .height("100%")
        .build()
        .unwrap();


    match Skim::run_with(options, Some(rx)) {
        Ok(out) if out.is_abort => return Err(anyhow!("No File Selected")),
        Ok(out) => skimoutput_to_string_single_selection(out),
        Err(e) => return Err(anyhow!("{}", e))
    } 

    // Okay I have a selected item as a string, just wrap in a Result and Option

}

fn skimoutput_to_string_single_selection(skim_output:SkimOutput) -> Result<String> {
    match skim_output.selected_items.iter().next() {
        Some(out) => Ok(out.output().into()),
        None => return Err(anyhow!("Skim Output to String Failed"))
    }
}


fn read_file(path: &Path ) -> Result<Option<String>> {
    let contents = std::fs::read_to_string(path);
        
    match contents {
        Ok(contents) => Ok(Some(contents)),
        Err(e) if e.kind() == ErrorKind::InvalidData => Ok(None),
        Err(e) => Err(e).with_context(|| format!("reading {}",path.display())),
    }
}

