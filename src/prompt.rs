use std::io::{stdout, Write};
use std::env;
use std::path::Path;

pub fn path_to_string(path: &Path) -> String {
    return path.to_str().unwrap().trim_matches('"').to_string();
}

fn generate_prompt() {
    let max_dir_length: usize = 3;
    let prompt_symbol = '❯';
    let current_dir = env::current_dir().unwrap();
    
    let dir: &Path;
    let dir_components = current_dir.components();
    let components_count = dir_components.clone().count();

    //match home::home_dir() {
    //    Some(x) => {
    //        let home_dir = path_to_string(x.as_path());
    //        let current_dir_str = path_to_string(&current_dir);

    //        match current_dir_str.strip_prefix(&home_dir) {
    //            Some(stripped_dir) => current_dir = "~".to_owned().push_str(&stripped_dir),
    //            None => ()
    //        }
    //    },
    //    None => (),
    //}

    if components_count > max_dir_length.into() {
        let mut components_iter = dir_components;
        for _ in 0..(components_count - max_dir_length) {
            components_iter.next();
        }
        dir = components_iter.as_path();
    }
    else {
        dir = current_dir.as_path();
    }

    print!("{} {} ", path_to_string(dir), prompt_symbol);
}

pub fn print_prompt() {
    //print!("{} {} ", path_to_string(env::current_dir().unwrap()), prompt_symbol);
    
    generate_prompt();

    match stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}
