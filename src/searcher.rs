// extern crate skim;
use skim::prelude::*;
use std::path::{Path, PathBuf};
use std::io::ErrorKind;
use ignore::Walk;

use anyhow::{Context, Result, anyhow};
use ratatui::text::Line;
use std::sync::Arc;

struct MultiLineItem {
    body: String,
    oneline: String,
    file_name: String
}

impl MultiLineItem {
    fn new(file_name:String, text:String ) -> Self {
        let body = format!("{}:\n{}",file_name,text);
        let oneline = body.replace('\n', "  ");
        Self {body , oneline, file_name}
    }
}

impl SkimItem for MultiLineItem {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.oneline)
    }
    fn preview(&self, _context: PreviewContext<'_>) -> ItemPreview {
        ItemPreview::Text(self.body.clone())

    }

    fn display(&self, _context: DisplayContext) -> Line<'_> {
        self.file_name.as_str().into()

    }
    fn output(&self) ->  Cow<'_, str> {
        Cow::Borrowed(&self.file_name)
    }
}

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
                    Ok(Some(contents)) => items.push(MultiLineItem::new(entry.path().display().to_string(),contents)),
                    Ok(None) => continue,
                    Err(e) => return Err(e.into())
                }
            },
                
            Err(err) => return Err(err.into()),
        }
    }
    let skimitems: Vec<Arc<dyn SkimItem>> = items.
        into_iter()
        .map(|item| Arc::new(item) as Arc<dyn SkimItem>)
        .collect();
    tx.send(skimitems)?;
    drop(tx);
    
    let options = SkimOptionsBuilder::default()
        .preview("echo {}")
        .preview_window("right:60%")
        .height("100%")
        .exact(true)
        .no_hscroll(true)
        .build()
        .unwrap();
    match Skim::run_with(options, Some(rx)) {
        Ok(out) if out.is_abort => return Err(anyhow!("No File Selected")),
        Ok(out) => skimoutput_to_string_single_selection(out),
        Err(e) => return Err(anyhow!("{}", e))
    } 
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

