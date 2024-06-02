use std::io::{stdout, Write};
use std::env;

fn path_buf_to_string(path: std::path::PathBuf) -> String {
    return path.into_os_string().into_string().unwrap().trim_matches('"').to_string();
}

pub fn print_prompt() {
    let prompt_symbol = '❯';
    print!("{} {} ", path_buf_to_string(env::current_dir().unwrap()), prompt_symbol);
    
    match stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}
