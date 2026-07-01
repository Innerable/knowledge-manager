use crate::commands::traits::Command;

pub struct Help;

impl Command for Help {
    fn get_command(&self) -> &'static str {
        "help"
    }
    fn process_command(&self, _args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        println!("The list of all possible commands: ");
        println!("help  - lists all possible commands");
        println!("index - updates index.md");
        println!("init  - Create an empty repository?");
        Ok(())
    }
}
