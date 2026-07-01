mod commands;
mod consts;

use commands::help::Help;
use commands::index::Index;
use commands::init::Init;
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
        Some(arg) if arg == Help.get_command() => Help.process_command(command_args),
        Some(arg) if arg == Init.get_command() => Init.process_command(command_args),
        Some(arg) if arg == Index.get_command() => Index.process_command(command_args),
        _ => Help.process_command(&[]),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {}", err);
            ExitCode::FAILURE
        }
    }
}
