use crate::commands::traits::Command;
use crate::consts::{DOCUMENTATION_ROOT, INDEX_ROOT, SPECS_ROOT};

use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Init;

impl Command for Init {
    fn get_command(&self) -> &'static str {
        "init"
    }
    fn process_command(&self, _args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        init()
    }
}

fn combine_to_string(path1: &str, path2: &str) -> String {
    format!("./{}/{}", path1, path2)
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(combine_to_string(DOCUMENTATION_ROOT, ""))?;
    let specs_example = include_bytes!("./specs_example.md");
    let mut index = File::create(combine_to_string(DOCUMENTATION_ROOT, INDEX_ROOT))?;
    let _ = File::create(combine_to_string(DOCUMENTATION_ROOT, SPECS_ROOT))?;
    fs::write(
        combine_to_string(DOCUMENTATION_ROOT, SPECS_ROOT),
        specs_example,
    )?;

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
        }
    }
    println!("init success");
    Ok(())
}
