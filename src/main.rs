use std::io::{stdin, stdout, Write};

mod cmds;

fn main() {
    let mut debug_mode: bool = false;

    loop {
        print!("WIN> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        // this needs to be peekable so we can determine when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();

        while let Some(command) = commands.next() {
            // everything after the first whitespace character is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts;

            if debug_mode {
                println!("{:?}", args);
            }

            match command {
                "hello" => {
                    crate::cmds::hello::cmd_hello();
                }
                "exit" | "quit" => {
                    println!("exit...");
                    return;
                }
                "debug" => {
                    debug_mode = !debug_mode;
                    println!("debug mode: {}", debug_mode);
                }
                "help" => {
                    crate::cmds::help::cmd_help();
                }
                _ => {}
            }
        }
    }
}

