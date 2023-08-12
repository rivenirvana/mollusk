use std::env;
use std::path::PathBuf;
pub fn change_dir(target_path: &str) -> Result<(), std::io::Error>{
    let home_dir = get_home_dir().expect("ERROR: Cannot find $HOME.");
    let mut root = PathBuf::new();

    if target_path.starts_with("~") {
        root.push(&home_dir);

        if target_path.len() >= 2 {
            root.push(&target_path[2..]);
        }
    } else {
        root.push(&target_path);
    }

    env::set_current_dir(&root)
}

pub fn get_home_dir() -> Option<String> {
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

pub fn getcwd() -> std::io::Result<PathBuf> {
    env::current_dir()
}
