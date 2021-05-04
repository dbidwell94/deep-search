use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

#[derive(Debug)]
enum TextSearch {
    Text(String),
    Expression(Regex),
}

#[derive(Debug)]
struct Args {
    path: String,
    search_query: TextSearch,
    exclude: Option<Regex>,
}

#[derive(Debug, Clone)]
struct DirWithIgnore {
    path: String,
    ignore: Option<Regex>,
}

fn main() {
    let mut mapped_args: HashMap<String, String> = HashMap::new();
    let mut system_args = env::args().collect::<VecDeque<String>>();

    while system_args.len() > 0 {
        let current_arg = system_args.pop_front().unwrap();
        if current_arg.starts_with('-') {
            match system_args.front() {
                None => {
                    mapped_args.insert(current_arg[1..].to_owned(), "".to_owned());
                }
                Some(next_string) => {
                    if next_string.starts_with('-') {
                        mapped_args.insert(current_arg[1..].to_owned(), "".to_owned());
                    } else {
                        let value = system_args.pop_front().unwrap();
                        mapped_args.insert(current_arg[1..].to_owned(), value);
                    }
                }
            }
        }
    }

    if !mapped_args.contains_key("t") && !mapped_args.contains_key("re") {
        panic!("Unable to continue, no text to search for is present!");
    }

    let search_path: String = if mapped_args.contains_key("p") {
        mapped_args.get("p").unwrap().to_owned()
    } else if mapped_args.contains_key("path") {
        mapped_args.get("path").unwrap().to_owned()
    } else {
        ".".to_owned()
    };

    let search_query: TextSearch = if mapped_args.contains_key("re") {
        let regex_text = mapped_args.get("re").unwrap();

        let mut re = regex::RegexBuilder::new(regex_text);

        if mapped_args.contains_key("f") {
            let regex_flags = mapped_args.get("f").unwrap().split("");
            for flag in regex_flags {
                if flag == "i" {
                    re.case_insensitive(true);
                } else if flag == "m" {
                    re.multi_line(true);
                }
            }
        }

        TextSearch::Expression(re.build().unwrap())
    } else {
        TextSearch::Text(mapped_args.get("t").unwrap().to_owned())
    };

    let exclude = if mapped_args.contains_key("e") {
        let exclude_list: Vec<String> = mapped_args
            .get("e")
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| format!("/{}/", s.to_owned()))
            .collect();
        let joined_list = exclude_list.join("|");
        Some(
            regex::RegexBuilder::new(&format!("({})", joined_list))
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    let args = Args {
        path: search_path,
        search_query,
        exclude,
    };

    do_dir_search(args);
}

fn do_dir_search(args: Args) {
    let mut dir_paths: VecDeque<DirWithIgnore> = VecDeque::new();
    dir_paths.push_front(DirWithIgnore {
        path: args.path.to_owned(),
        ignore: check_dir_for_gitignore(&args.path),
    });

    let mut gitignore_parent_path: Option<DirWithIgnore> = None;

    while dir_paths.len() > 0 {
        let current_path = dir_paths
            .pop_front()
            .expect("Unable to pop path from deque");

        if gitignore_parent_path.is_none() && current_path.ignore.is_some() {
            gitignore_parent_path = Some(current_path.to_owned());
        } else if gitignore_parent_path.is_some()
            && current_path.ignore.is_some()
            && !gitignore_parent_path
                .as_ref()
                .unwrap()
                .path
                .contains(&current_path.path)
        {
            gitignore_parent_path = Some(current_path.to_owned());
        }

        if matches!(fs::metadata(&current_path.path), Ok(val) if val.is_dir() || val.is_file())
            && args
                .exclude
                .as_ref()
                .map_or(true, |re| !re.is_match(&current_path.path))
            && gitignore_parent_path.as_ref().map_or(true, |dir| {
                dir.ignore
                    .as_ref()
                    .map_or(true, |re| !re.is_match(&current_path.path))
            })
        {
            match fs::read_dir(&current_path.path) {
                Ok(dir) => {
                    for sub_dir in dir {
                        let new_path = sub_dir.unwrap().path().to_str().unwrap().to_owned();
                        dir_paths.push_front(DirWithIgnore {
                            ignore: check_dir_for_gitignore(&new_path),
                            path: new_path,
                        });
                    }
                }
                Err(_) => match fs::read_to_string(&current_path.path) {
                    Ok(string) => {
                        let mut line_number = 0;
                        for line in string.split('\n') {
                            line_number += 1;

                            match &args.search_query {
                                TextSearch::Text(search_text) => {
                                    do_text_search(
                                        line,
                                        search_text,
                                        &line_number,
                                        &current_path.path,
                                    );
                                }
                                TextSearch::Expression(regex) => {
                                    do_regex_search(line, regex, &line_number, &current_path.path);
                                }
                            }
                        }
                    }
                    Err(_) => {}
                },
            }
        }
    }
}

fn do_text_search(line_to_search: &str, text_to_search_for: &str, line_number: &i32, path: &str) {
    if line_to_search.contains(text_to_search_for) {
        println!("line: {:?}, path: {:?}", line_number, path);
    }
}

fn do_regex_search(line_to_search: &str, regex: &Regex, line_number: &i32, path: &str) {
    if regex.is_match(line_to_search) {
        println!("line: {:?}, path: {:?}", line_number, path);
    }
}

fn check_dir_for_gitignore(path: &str) -> Option<Regex> {
    match fs::read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                if matches!(&entry, Ok(f) if f.file_name().to_str().unwrap() == ".gitignore") {
                    return Some(parse_gitignore(&entry.unwrap().path().to_str().unwrap()));
                }
            }
            return None;
        }
        Err(_) => None,
    }
}

fn parse_gitignore(gitignore_path: &str) -> Regex {
    let gitignore_string = fs::read_to_string(gitignore_path).unwrap();

    let split_gitignore = gitignore_string
        .split('\n')
        .map(|s| {
            format!(
                "{}",
                s.replace("|", r"\|")
                    .replace("*", "")
                    .replace(".", r"\.")
                    .replace("^", r"\^")
                    .replace("(", r"\(")
                    .replace(")", r"\)")
                    .replace("/", "")
                    .trim()
            )
        })
        .filter(|s| !s.starts_with('#') && s.len() > 0)
        .collect::<Vec<String>>()
        .join("|");

    let re = regex::RegexBuilder::new(&format!("({})", split_gitignore))
        .build()
        .unwrap();

    return re;
}
