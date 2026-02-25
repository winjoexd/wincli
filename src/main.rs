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

        while let Some(full_cmd) = commands.next() {
            // everything after the first whitespace character is interpreted as the single param string
            let mut parts = full_cmd.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let param = full_cmd.trim().splitn(2, char::is_whitespace).nth(1).unwrap_or("").to_string();

            if debug_mode {
                println!("{:?}", param);
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
                "param" => {
                    crate::cmds::param::cmd_param(param);
                }
                _ => {}
            }
        }
    }
}

