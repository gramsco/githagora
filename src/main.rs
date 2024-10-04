use std::{
    env,
    path::Path,
};

mod git;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <directory> <command> [<arg>...]");
        return;
    }

    let directory = &args[1]; // First argument (directory)
    let command = &args[2]; // Second argument (command to run)
    let command_args = &args[3..]; // Remaining arguments (passed to the command)

    let path = Path::new(&directory);
    env::set_current_dir(path).expect("Could not set current dir.");

    let git = git::Git::new();
    let _ = git.bisect(command, &command_args);
}
