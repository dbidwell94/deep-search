use std::env;
use std::collections::{HashMap,VecDeque};
use std::fs;

fn main() {

    let mut args: HashMap<String, String> = HashMap::new();

    let mut dir_paths: VecDeque<String> = VecDeque::new();

    for arg in env::args() {
            if arg.len() > 1 && arg.starts_with('-') && arg[1..arg.len() - 1].contains('=') {
                let split_arg = arg[1..].split('=').collect::<Vec<&str>>();

                let (key, value): (&str, &str) = (split_arg[0], split_arg[1]);
                args.insert(key.to_owned().to_lowercase(), value.to_owned());
            }
    }


    if args.contains_key("path") && args.contains_key("text") {
        dir_paths.push_back(args.get("path").unwrap().to_owned());
    }

    while dir_paths.len() > 0 {
        let current_path = dir_paths.pop_front().expect("Unable to pop path from deque");
        match fs::read_dir(&current_path) {
            Ok(dir) => {
                for sub_dir in dir {
                    let new_path = sub_dir.unwrap().path().to_str().unwrap().to_owned();
                    dir_paths.push_back(new_path);
                }
            }
            Err(_) => {
                match fs::read_to_string(&current_path) {
                    Ok(string) => {
                        let mut line_number = 0;
                        for line in string.split('\n') {
                            line_number += 1;
                            if line.contains(args.get("text").unwrap()) {
                                println!("line: {}, path: {}", &line_number, &current_path);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
}