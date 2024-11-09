use std::io::{IsTerminal, Write};
use std::{env, io, process};

fn main() {
    loop {
        show_prompt();
        for command in read_line() {
            command.run();
        }
    }
}

fn show_prompt() {
    let mut stdout = io::stdout();
    if stdout.is_terminal() {
        print!("> ");
        stdout.flush().unwrap();
    }
}

fn read_line() -> Vec<Command> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.split(';').map(|s| s.to_string().into()).collect()
}

#[derive(Debug)]
struct Command {
    binary: Option<String>,
    args: Vec<String>,
}

impl Command {
    fn run(&self) {
        if let Some(binary) = self.binary.clone() {
            match binary.as_str() {
                "cd" => self.cd(),
                "exit" => process::exit(0),
                binary => {
                    process::Command::new(binary)
                        .args(self.args.clone())
                        .status()
                        .expect("Error while running process");
                }
            };
        }
    }

    fn cd(&self) {
        match self.args.len() {
            1 => {
                let current = env::current_dir().expect("Unable to get working directory");
                let new = current.join(self.args[0].clone());
                env::set_current_dir(new).unwrap_or_else(|e| eprintln!("{e}"));
            }
            _ => eprintln!("Expected exactly one argument"),
        }
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        let mut split = value.trim().split_whitespace().map(|s| s.to_string());

        Self {
            binary: split.next(),
            args: split.collect(),
        }
    }
}
