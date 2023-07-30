use std::env;
use std::io;
use std::io::Write;
use std::path::{PathBuf};
use std::process::{Command, Stdio};

struct CommandChain<T> {
    commands: Vec<T>,
    operators: Vec<T>,
    arguments: Vec<Vec<T>>
}

fn main() {
    loop {
        // check for existence of $HOME. Fail hard if it doesn't exist.
        let home_dir = get_home_dir().expect("ERROR: Cannot find $HOME.");

        let prompt_string = create_prompt();
        print!("{}", prompt_string);
        io::stdout().flush().unwrap();

        let mut input_cmd = String::new();
        io::stdin()
            .read_line(&mut input_cmd)
            .expect("failed to read stdin");

        let cmd_tokens = get_command_tokens(&input_cmd);
        let cmd_chain = parse_cmd_tokens(cmd_tokens);
        
        for (idx, cmd) in cmd_chain.commands.iter().enumerate() {
            let args = &cmd_chain.arguments[idx];

            match *cmd {
                "exit" => return,
                "cd" => {
                    let target_path = args[0];
                    let mut root = PathBuf::new();

                    if target_path.starts_with("~") {
                        root.push(&home_dir);

                        if target_path.len() >= 2 {
                            root.push(&target_path[2..]);
                        }
                    } else {
                        root.push(&target_path);
                    }

                    match env::set_current_dir(&root) {
                        Err(e) => { eprintln!("{}", e) },
                        _ => ()
                    }
                },
                cmd => {
                    let output = Command::new(cmd)
                        .args(args)
                        .stdout(Stdio::inherit())
                        .spawn();

                    match output {
                        Ok(mut output) => {
                            let _ = output.wait();

                            if let Some(op) = cmd_chain.operators.get(idx) {
                                match *op {
                                    "&&" => { continue; },
                                    "||" => { break; },
                                    _ => (),
                                }
                            }

                            continue;
                        },
                        Err(e) => {

                            eprintln!("{}", e);
                            
                            if let Some(op) = cmd_chain.operators.get(idx) {
                                match *op {
                                    "&&" => { break; },
                                    "||" => { continue; },
                                    _ => (),
                                }
                            }

                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn parse_cmd_tokens(tokens: Vec<&str>) -> CommandChain<&str> {
    let chain_ops: [&str; 2] = ["&&", "||"];
    
    let mut commands: Vec<&str> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();
    let mut arguments: Vec<Vec<&str>> = vec![vec![]];
    
    let mut token_iter = tokens.iter();

    let mut prev_token: &str = "";

    while let Some(token) = token_iter.next() { 
        if commands.len() == 0 {
            commands.push(token);
            continue;
        }

        if chain_ops.contains(token) {
            operators.push(token);
            prev_token = token;
            continue;
        }

        if chain_ops.contains(&prev_token) {
            commands.push(token);
            arguments.push(vec![]);
        } else {
            arguments[commands.len() - 1].push(token);
        }

        prev_token = token;
    };

    CommandChain {
        commands,
        arguments,
        operators
    }
}

fn get_command_tokens(input_cmd: &str) -> Vec<&str> {
    let mut commands: Vec<&str> = input_cmd.trim().split_whitespace().collect();

    for command in commands.iter_mut() {
        *command = command.trim();
    }

    return commands;
}

fn create_prompt() -> String {
    let cwd = getcwd().unwrap();

    // fail hard if none of the environment variables exist
    let home = env::var("HOME").expect("ERROR: Cannot find $HOME.");
    let user = env::var("USER").expect("ERROR: Cannot find $USER.");
    let name = env::var("NAME").expect("ERROR: Cannot find $NAME.");

    return format!(
        "[{}@{} {}]$ ", 
        user, 
        name, 
        cwd.into_os_string().into_string().unwrap().replace(&*home, "~")
    );
}

fn get_home_dir() -> Option<String> {
    // we implement it as a custom function 
    // since env::home_dir is deprecated

    match env::var("HOME") {
        Ok(home_dir) => Some(home_dir),
        Err(e) => { 
            eprintln!("{}", e);
            None
        }
    }
}

fn getcwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}
