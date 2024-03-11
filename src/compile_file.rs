use crate::acceseories::full_file_read as read_file;  // Must be a reference of a string for a path.
use crate::acceseories::full_file_write as create_file;
use crate::acceseories::run_bash_command as shell_command;
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
    ouput_code += "use std::io;use std::thread;pub fn get_input() -> String {let mut input = String::from(\"\");io::stdin().read_line(&mut input).expect(\"Failed to read line\");return input;}pub fn main(){";

    println!("Ouput file : {}", output_path);

    println!("Starting code added."); // Unecisary code to make me look cool in front of noobs

    let mut iter = 1;
    println!("Iter created.");

    let mut variable_list: Vec<String> = vec![String::from("")];
    println!("Variable list created.");

    for line in code_lines {
        let token_code = tokenize(line.clone());

        ouput_code = commands::print(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone(), variable_list.clone());  // 'ouput code =' not '+=' because print function does +=
        ouput_code = commands::wait(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone(), variable_list.clone());
        ouput_code = (commands::variable(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone(), variable_list.clone())).0;
        ouput_code = (commands::file_end(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone())).0;
        ouput_code = commands::add(token_code.clone(), ouput_code.clone());
        ouput_code = commands::subtract(token_code.clone(), ouput_code.clone());
        ouput_code = commands::multiply(token_code.clone(), ouput_code.clone());
        ouput_code = commands::divide(token_code.clone(), ouput_code.clone());
        ouput_code = commands::readln(token_code.clone(), ouput_code.clone());

        if (commands::variable(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone(), variable_list.clone())).2 {
            variable_list.push((commands::variable(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone(), variable_list.clone())).1);
            println!("Variable list updated.");
        }

        if (commands::file_end(token_code.clone(), ouput_code.clone(), iter.clone(), line.clone())).1 {
            break;
        }

        iter += 1;  // Just the iter. Idk why I made a comment
    }

    println!("\n\n\nResulting rust code:\n\n{}", ouput_code);

    println!("Creating cache rust file...");

    create_file(String::from("blt_cache.rs"), ouput_code);
    shell_command(format!("rustc blt_cache.rs -o {}", output_path));
}