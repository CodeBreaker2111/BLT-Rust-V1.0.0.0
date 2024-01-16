pub fn print(token_code: Vec<String>, ouput_code: String, iter: i32, line: String, variables: Vec<String>) -> String {
    let mut ouput_code_clone = ouput_code.clone();
    
    if token_code[0] == "print" {

        if token_code[1] == "string" {
            if token_code.len() > 4 {
                let mut print = String::from(""); // To keep track of spaces in code because of tokenazation
                let mut iteration = 0;

                for i in &token_code {
                    iteration += 1;

                    if iteration >= 4 {
                        print = print + " " + &i;
                    }
                }

                ouput_code_clone += format!("println!(\"{}\");", print.trim_start()).as_str();  // as str cause format returns String
                println!("ouput_code updated; print")
            }
            else {
                if token_code[3] == "\n" {
                    ouput_code_clone += "println!(\"\");";
                    println!("ouput_code updated; print (\\n)");
                }

                else {
                    ouput_code_clone += format!("println!(\"{}\");", token_code[3]).as_str();
                }
            }
        }

        if token_code[1] == "var" {
            ouput_code_clone += format!("println!(\"{{}}\", v{}.as_str());", token_code[2]).as_str();
            println!("ouput_code updated; print (var)");
            println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being not string.", iter.to_string(), line);

            // Error check

            if !variables.contains(&token_code[2]) {
                eprintln!("\n\nERROR line {} :\n'{}' The variable '{}' does not exist.\nAborting compile...", iter, line, token_code[2]);
                std::process::exit(1);
            }
        }
    }

    return ouput_code_clone;
}

pub fn wait(token_code: Vec<String>, ouput_code: String, iter: i32, line: String, variables: Vec<String>) -> String {
    let mut ouput_code_clone = ouput_code.clone();
    
    if token_code[0] == "wait" {
        if token_code[2] == "ms" {
            ouput_code_clone += format!("thread::sleep_ms({});", token_code[1]).as_str();
            println!("ouput_code updated; wait");
        }

        if token_code[1] == "var" {
            ouput_code_clone += format!("thread::sleep_ms(v{});", token_code[2]).as_str();
            println!("ouput_code updated; wait (var)");
            println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or not being int.", iter.to_string(), line);

            // Error check

            if !variables.contains(&token_code[2]) {
                eprintln!("\n\nERROR line {} :\n'{}' The variable '{}' does not exist.\nAborting compile...", iter, line, token_code[2]);
                std::process::exit(1);
            }
        }
    }

    return ouput_code_clone;
}

pub fn variable(token_code: Vec<String>, ouput_code: String, iter: i32, line: String, variables: Vec<String>) -> (String, String, bool) {
    let mut ouput_code_clone = ouput_code.clone();
    let mut variable_created_bool = false;
    let mut variable_created = String::from("");
    let mut exists = false;

    if token_code[0] == "var" {
        if variables.contains(&token_code[2]) {
            exists = true;
        }

        if token_code[2] == "int" {
            if !exists {
                ouput_code_clone += format!("let mut v{} = {};", token_code[2], token_code[3]).as_str();
                println!("ouput_code updated; var (not-exists)");

                variable_created_bool = true;
                variable_created = token_code[2].clone();

                // Not allowed: !  -  @  #  /  \  $  %  ^  ;  :  "  '  [  ]  {  }  ,  .  ?  =  +  *  (  )  `  ~

                if token_code[2].as_str().contains('!') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : !\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('-') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : -\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('@') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : @\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('#') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : #\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('/') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : /\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\\') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \\\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('$') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : $\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('%') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : %\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('^') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ^\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(';') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ;\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(':') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : :\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('"') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \"\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\'') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : '\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('[') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : [\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(']') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ]\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('{') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : {{\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('}') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : }}\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(',') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ,\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('.') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : .\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('?') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ?\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('=') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : =\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('+') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : +\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('*') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : *\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('(') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : (\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(')') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : )\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('`') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : `\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('~') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ~\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
            }

            if exists {
                ouput_code_clone += format!("v{} = {}", token_code[2], token_code[3]).as_str();
                println!("ouput_code updated; var (exists)");
                println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being wrong type", iter.to_string(), line);

                // Error check

                if !variables.contains(&token_code[2]) {
                    eprintln!("\n\nERROR line {} :\n'{}' Variable {} does not exist.\nAborting compile...", iter, line, token_code[3]);
                    std::process::exit(1);
                }
            }
        }

        if token_code[1] == "string" {
            if token_code.len() > 5 {

                let mut var_string = String::from("");  // Current variable; finding value of string due to tokenazation spliting up string
                let mut iteration = 0;
                println!("Iterator line {} created", iter.to_string());

                for i in &token_code {
                    iteration += 1;

                    if iteration >= 5 {
                        var_string += " "; var_string += i;
                    }
                }

                if !exists {
                    ouput_code_clone += format!("let mut v{} = String::from(\"{}\");", token_code[2], var_string).as_str();
                    println!("ouput_code updated; var (not-exists)");

                    variable_created_bool = true;
                    variable_created = token_code[2].clone();

                    if token_code[2].as_str().contains('!') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : !\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('-') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : -\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('@') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : @\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('#') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : #\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('/') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : /\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\\') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \\\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('$') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : $\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('%') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : %\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('^') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ^\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(';') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ;\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(':') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : :\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('"') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \"\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\'') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : '\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('[') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : [\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(']') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ]\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('{') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : {{\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('}') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : }}\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(',') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ,\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('.') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : .\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('?') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ?\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('=') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : =\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('+') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : +\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('*') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : *\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('(') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : (\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(')') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : )\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('`') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : `\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('~') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ~\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }

                }
    
                if exists {
                    ouput_code_clone += format!("v{} = String::from(\"{}\");", token_code[2], var_string).as_str();
                    println!("ouput_code updated; var (exists)");
                    println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being wrong type", iter.to_string(), line);

                    // Error check

                    if !variables.contains(&token_code[2]) {
                        eprintln!("\n\nERROR line {} :\n'{}' Variable {} does not exist.\nAborting compile...", iter, line, token_code[3]);
                        std::process::exit(1);
                    }
                }
            }

            else {
                if !exists {
                    ouput_code_clone += format!("let mut v{} = String::from(\"{}\");", token_code[2], token_code[3]).as_str();
                    println!("ouput_code updated; var (not-exists)");

                    variable_created_bool = true;
                    variable_created = token_code[2].clone();

                    if token_code[2].as_str().contains('!') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : !\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('-') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : -\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('@') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : @\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('#') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : #\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('/') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : /\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\\') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \\\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('$') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : $\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('%') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : %\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('^') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ^\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(';') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ;\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(':') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : :\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('"') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : \"\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('\'') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : '\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('[') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : [\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(']') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ]\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('{') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : {{\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('}') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : }}\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(',') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ,\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('.') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : .\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('?') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ?\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('=') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : =\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('+') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : +\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('*') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : *\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('(') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : (\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains(')') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : )\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('`') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : `\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                    if token_code[2].as_str().contains('~') { eprintln!("\n\nERROR line {}:\n'{}'\nThe variable : '{}' contains an illegal character : ~\nAborting compile...", iter, line, token_code[2]); std::process::exit(1); }
                }
    
                if exists {
                    ouput_code_clone += format!("v{} = String::from(\"{}\");", token_code[2], token_code[3]).as_str();
                    println!("ouput_code updated; var (exists)");
                    println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being wrong type", iter.to_string(), line);

                    // Error check

                    if !variables.contains(&token_code[2]) {
                        eprintln!("\n\nERROR line {} :\n'{}' The variable '{}' does not exist.\nAborting compile...", iter, line, token_code[2]);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    return (ouput_code_clone, variable_created, variable_created_bool);
}

pub fn file_end(token_code: Vec<String>, ouput_code: String, iter: i32, line: String) -> (String, bool) {
    if token_code[0] == "break" || token_code[0] == "}" {
        println!("file end detected line {} :\n'{}'\nouput_code updated; file-end", iter, line);
        return (ouput_code + "}", true);
    }
    else {
        return (ouput_code, false)
    }
}

pub fn add(token_code: Vec<String>, ouput_code: String, iter: i32, line: String) -> (String, bool) {
    let mut ouput_code_clone = ouput_code.clone();
    
    if token_code[0] == "add" {
        ouput_code_clone = ouput_code_clone + format!("let mut v{} =", token_code[5]).as_str();

        if token_code[1] == "int" {
            ouput_code_clone = ouput_code_clone + format!(" {}", token_code[2]).as_str();
        }
        if token_code[1] == "variable" {
            ouput_code_clone = ouput_code_clone + format!(" v{}", token_code[2]).as_str();
        }

        ouput_code_clone = ouput_code_clone + " +";

        if token_code[3] == "int" {
            ouput_code_clone = ouput_code_clone + format!(" {}", token_code[4]).as_str();
        }
        if token_code[3] == "variable" {
            ouput_code_clone = ouput_code_clone + format!(" v{}", token_code[4]).as_str();
        }
    }

    return (ouput_code_clone, false);
}