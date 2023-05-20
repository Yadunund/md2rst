use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn help() {
    println!(
        "usage:
md2rst <PATH_TO_CHANGELOG.md>
    Convert the CHANGELOG.md into a CHANGELOG.rst"
    );
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // one argument passed
        2 => {
            let changelog_path = &args[1];
            println!("The input path is {}", changelog_path);
            let path = Path::new(&changelog_path);
            match path.exists() {
                false => {
                    println!("The file does not exist!");
                    return;
                }
                true => {
                    let file_as_string = match fs::read_to_string(path) {
                        Ok(string) => string,
                        Err(_e) => {
                            println!("Unable to read file as string.");
                            return;
                        }
                    };
                    println!("Successfully read file into string: {}", file_as_string);
                }
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
