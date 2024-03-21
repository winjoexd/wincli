use std::io::{stdin, stdout, Write};

mod cmds;

fn main() {
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
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "hello" => {
                    crate::cmds::hello::cmd_hello();
                }
                "exit" => return, 
                _ => {}
            }
        }
    }
}
