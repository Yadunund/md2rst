/*
 * Copyright (C) 2023 Yadunund Vijay
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use markdown::{to_mdast, ParseOptions};

fn help() {
    println!(
        "usage:
md2rst INPUT_PATH OUTPUT_PATH
    Convert a CHANGELOG.md into a CHANGELOG.rst"
    );
}

fn convert(input: String, output: Option<String>) -> () {
    println!("The input path is {}", input);
    let input_path = Path::new(&input);
    let output_file = match output {
        Some(path) => {
            println!("Using provided output path: {}", path);
            path
        }
        None => {
            println!("No output path provided. Defaulting to: CHANGELOG.rst");
            String::from("CHANGELOG.rst")
        }
    };
    let output_path = Path::new(&output_file);

    match input_path.exists() {
        false => {
            println!("The input file does not exist!");
            return;
        }
        true => {
            let file_as_string = match fs::read_to_string(input_path) {
                Ok(string) => string,
                Err(_e) => {
                    println!("Unable to read file as string.");
                    return;
                }
            };
            let md_tree = to_mdast(&file_as_string, &ParseOptions::default()).unwrap();
            println!("Successfully parsed markdown into tree: \n{:?}", md_tree);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // one argument passed
        2 => {
            convert(args[1].to_owned(), None);
        }
        3 => {
            convert(args[1].to_owned(), Some(args[2].to_owned()));
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
