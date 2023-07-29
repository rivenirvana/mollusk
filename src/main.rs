use std::env;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
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

        let commands = get_commands(&input_cmd);

        for command in commands.iter() {
            let tokens = get_command_tokens(command);
            let command_name = &*tokens.name;

            match command_name {
                "exit" => return,
                "cd" => {
                    let root = Path::new(tokens.arguments[0]);
                    match env::set_current_dir(&root) {
                        Err(e) => eprintln!("{}", e),
                        _ => ()
                    }
                },
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

fn get_commands(input_cmd: &str) -> Vec<&str>{
    let mut commands: Vec<&str> = input_cmd.trim().split(";").collect();

    for command in commands.iter_mut() {
        *command = command.trim();
    }

    println!("{:?}", &commands);

    return commands;
}

fn create_prompt() -> String {
    let cwd = getcwd().unwrap();

    return format!("{}>", cwd.display());
}

fn getcwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}
