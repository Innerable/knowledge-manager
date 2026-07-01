use crate::commands::traits::Command;
use crate::consts::{DOCUMENTATION_ROOT, INDEX_ROOT};

use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Index;

impl Command for Index {
    fn get_command(&self) -> &'static str {
        "index"
    }
    fn process_command(&self, _args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        index_update()
    }
}

fn combine_to_string(path1: &str, path2: &str) -> String {
    format!("./{}/{}", path1, path2)
}

pub fn index_update() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = File::create(combine_to_string(DOCUMENTATION_ROOT, INDEX_ROOT))?;
    let mut description = String::new();
    for path in fs::read_dir(combine_to_string(DOCUMENTATION_ROOT, "")).unwrap() {
        let temp_file = path.unwrap().path().display().to_string();
        if temp_file != combine_to_string(DOCUMENTATION_ROOT, INDEX_ROOT) {
            for line in fs::read_to_string(&temp_file)?.lines() {
                if let Some((_before, after)) = line.split_once("description:") {
                    description.push_str(after.trim());
                    break;
                }
            }
            writeln!(index, "{} {}", temp_file, &description)?;
            println!("{} {} inserted into index.md", temp_file, &description);
        }
    }
    Ok(())
}
