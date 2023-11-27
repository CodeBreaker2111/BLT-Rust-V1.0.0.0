use crate::acceseories::full_file_read as read_file;  // Must be a reference of a string for a path.
use crate::commands as commands;

pub fn main(input_path: String, output_path: String) {
    print!("\n\n");
    println!("Compiling {} into {}...", input_path.replace('\n', ""), output_path.replace('\n', ""));  // Replace because of some bug of \n's in string``

    let read = read(input_path, output_path);

    compile(read.0, read.1);
}

fn read(input_path: String, output_path: String) -> (Vec<String>, String) {
    println!("Reading file {}.", input_path);

    let mut result: (Vec<String>, String) = (vec![String::from("")], String::from(""));


    let input_code = read_file(&input_path);  // Code from file input_path
    println!("{}", input_code);

    let processed_lines: Vec<&str> = input_code  // Is a string slice not string; will be converted: line 24
        .lines()
        .filter(|line| line.starts_with(';'))
        .map(|line| line.trim_start_matches(';').trim())
        .collect();

    let input_code_lines: Vec<String> = processed_lines.iter().map(|&s| s.to_string()).collect();
    
    result.0 = input_code_lines;
    result.1 = output_path;

    return result;
}

fn tokenize(line: String) -> Vec<String> {
    let tokens_str: Vec<&str> = line.split_whitespace().collect();
    let tokens: Vec<String> = tokens_str.iter().map(|&s| s.to_string()).collect();

    return tokens;
}

fn compile(code_lines: Vec<String>, output_path: String) {
    let mut ouput_code = String::from("");
    ouput_code += "use std::io;use std::thread;pub fn main(){";

    println!("Ouput file : {}", output_path);

    println!("Starting code added."); // Unecisary code to make me look cool in front of noobs

    let mut iter = 1;
    println!("Iter created.");

    for line in code_lines {
        let token_code = tokenize(line.clone());

        ouput_code = commands::print(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone());  // 'ouput code =' not '+=' because print function does +=
        ouput_code = commands::wait(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone());
        ouput_code = commands::variable(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone());

        iter += 1;  // Just the iter. Idk why I made a comment
    }

    println!("{}", ouput_code);
}