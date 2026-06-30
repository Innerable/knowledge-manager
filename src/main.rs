mod commands;
mod consts;

use commands::help::Help;
use commands::traits::Command;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if (args.get(1) == Some(&Help.get_command().to_string())) || (args.get(1) == None) {
        let _ = Help.process_command(&[]);
        return ExitCode::SUCCESS;
    }
    let command_args = if args.len() > 2 { &args[2..] } else { &[] };
    let result = match args.get(1) {
        Some(cmd) if cmd == Help.get_command() => Help.process_command(command_args),
        _ => {
            let _ = Help.process_command(&[]);
            return ExitCode::SUCCESS;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {}", err);
            ExitCode::FAILURE
        }
    }
}
