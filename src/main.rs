use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    loop {
        let prompt_string = create_prompt();
        print!("{}", prompt_string);

        io::stdout().flush().unwrap();

        let mut input_cmd = String::new();
        io::stdin().read_line(&mut input_cmd)
            .expect("failed to read stdin");

        let command = input_cmd.trim();

        match command {
            "exit" => return,
            command => {
                let output = Command::new(command).spawn();

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

fn create_prompt() -> String {
    let cwd = getcwd().unwrap();

    return format!("{}>", cwd.display());
}

fn getcwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}
