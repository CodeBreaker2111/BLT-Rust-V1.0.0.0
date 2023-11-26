use crate::acceseories::full_file_read as read_file;  // Must be a reference of a string for a path.

pub fn main(input_path: String, output_path: String) {
    print!("\n\n");
    println!("Compiling {} into {}...", input_path.replace('\n', ""), output_path.replace('\n', ""));  // Replace because of some bug of \n's in string``

    read(input_path, output_path);
}

fn read(input_path: String, output_path: String) {
    println!("Reading file {}.", input_path);

    let input_code = read_file(&input_path);  // Code from file input_path
    println!("{}", input_code);

    let processed_lines: Vec<&str> = input_code  // Is a string slice not string; will be converted: line 24
        .lines()
        .filter(|line| line.starts_with(';'))
        .map(|line| line.trim_start_matches(';').trim())
        .collect();

    let input_code_lines: Vec<String> = processed_lines.iter().map(|&s| s.to_string()).collect();
}

fn tokenize(line: String) -> Vec<String> {
    let tokens_str: Vec<&str> = line.split_whitespace().collect();
    let tokens: Vec<String> = tokens_str.iter().map(|&s| s.to_string()).collect();

    return tokens;
}

fn compile(code_lines: Vec<String>) {
    let mut ouput_code = String::from("");
    ouput_code += "use std::io;pub fn main(){";

    println!("Starting code added."); // Unecisary code to make me look cool in front of noobs

    let mut variables: Vec<String> = Vec::new();

    for line in code_lines {
        let token_code = tokenize(line);

        if token_code[0] == "print" {

            if token_code[1] == "string" {
                if token_code.len() > 4 {
                    let mut print = String::from(""); // To keep track of spaces in code because of tokenazation
                    let mut iteration = 0;

                    for i in token_code {
                        iteration += 1;

                        if iteration >= 4 {
                            print = print + " " + &i;
                        }
                    }

                    ouput_code += format!("println!(\"{}\");", print).as_str();  // as str cause format returns String
                    println!("ouput_code updated; print")
                }
                else {
                    if token_code[3] == "\n" {
                        ouput_code += "println(\"\");";
                        println!("ouput_code updated; print (\\n)");
                    }
                }
            }

            if token_code  // left off at line 62 of python file
        }
    }
}