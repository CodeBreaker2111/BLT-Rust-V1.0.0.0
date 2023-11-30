use std::io;

pub fn get_input() -> String {
    let mut input = String::from("");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

use std::fs;

fn read_file(input_path: &String) -> Result<String, Box<dyn std::error::Error>> {
    match fs::read_to_string(input_path.replace('\n', "")) {
        Ok(contents) => {
            println!("Successfully read file {}", input_path);
            Ok(contents)
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

pub fn full_file_read(input_path: &String) -> String {
    let contents = match read_file(input_path) {
        Ok(contents) => contents,
        Err(err) => {
            // Handle the error here if needed
            eprintln!("Failed to read file {}: {}", input_path, err);
            std::process::exit(1);
        }
    };

    return contents;
}