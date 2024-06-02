use std::io::stdin;
use std::process::Command;
use std::path::Path;
use std::env;

mod prompt;

fn main_loop() {
    prompt::print_prompt();
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
   
    if input.trim().is_empty() {
        return;
    }

    // read_line leaves a trailing newline, which trim removes
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap();

    let args = parts;

    match command {
        "cd" => {
            let new_dir = args.peekable().peek().map_or("/", |x| *x);

            let home_dir: String;
            match home::home_dir() {
                Some(x) => home_dir = prompt::path_to_string(x.as_path()),
                None => home_dir = "/".to_string(),
            }

            let new_dir2 = new_dir.replace("~", &home_dir).to_string();

            let root = Path::new(&new_dir2);
            if let Err(e) = env::set_current_dir(root) {
                eprintln!("{}", e);
            }
        },
        "exit" => std::process::exit(0),
        command => {
            let child = Command::new(command)
                .args(args)
                .spawn();

            match child {
                Ok(mut child) => { let _ = child.wait(); },
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}

fn main(){
    loop {
        main_loop();
    }
}
