use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct CommandTokens<T> {
    name: String,
    arguments: Vec<T>
}

fn main() {
    loop {
        let prompt_string = create_prompt();
        print!("{}", prompt_string);

        io::stdout().flush().unwrap();

        let mut input_cmd = String::new();
        io::stdin().read_line(&mut input_cmd)
            .expect("failed to read stdin");

        let tokens = get_command_tokens(&input_cmd);
        let command_name = &*tokens.name;

        match command_name {
            "exit" => return,
            command_name => {
                let output = Command::new(command_name)
                    .args(tokens.arguments)
                    .stdout(Stdio::inherit())
                    .spawn();

                match output {
                    Ok(mut output) => {
                        let _ = output.wait();
                    },
                    Err(e) => eprintln!("{}", e)
                }
            }
        }
    }
}

fn get_command_tokens(input_cmd: &str) -> CommandTokens<&str> {
    let tokens: Vec<&str> = input_cmd.trim().split(" ").collect();
    let mut arguments: Vec<&str> = Vec::with_capacity(tokens.len() - 1);
    arguments.append(&mut tokens[1..].to_vec());

    CommandTokens {
        name: tokens[0].to_string(),
        arguments
    }
}

fn create_prompt() -> String {
    let cwd = getcwd().unwrap();

    return format!("{}>", cwd.display());
}

fn getcwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}
