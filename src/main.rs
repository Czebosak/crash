use std::{
    io::{stdin, stdout, Write},
    process::Command, path::Path, env};

fn main(){
    loop {
        print!("{} > ", env::current_dir().unwrap().into_os_string().into_string().unwrap().trim_matches('"'));
        match stdout().flush() {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                let _ = child.wait();
            }
        } 
    }
}