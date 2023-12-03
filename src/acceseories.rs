use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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

fn file_write(path: String, contents: String) -> std::io::Result<()> {
    let mut file = File::create(path.as_str())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn full_file_write(path: String, contents: String) {
    let _ = file_write(path, contents);
}

pub fn run_bash_command(command: String) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");
}