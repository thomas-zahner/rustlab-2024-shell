use std::io::Write;
use std::{io, process};

fn main() {
    loop {
        show_prompt();
        let command = read_line();

        process::Command::new(command.binary)
            .args(command.args)
            .status()
            .expect("Error");
    }
}

fn show_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn read_line() -> Command {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop(); // pop the newline
    buffer.into()
}

#[derive(Debug)]
struct Command {
    binary: String,
    args: Vec<String>,
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let split: Vec<_> = value.split_whitespace().map(|s| s.to_string()).collect();

        if split.len() == 0 {
            todo!()
        }

        Self {
            binary: split[0].clone().into(),
            args: split[1..].to_vec(),
        }
    }
}
