pub fn print(token_code: Vec<String>, ouput_code: String, iter: i32, line: String) -> String {
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

                ouput_code_clone += format!("println!(\"{}\");", print).as_str();  // as str cause format returns String
                println!("ouput_code updated; print")
            }
            else {
                if token_code[3] == "\n" {
                    ouput_code_clone += "println(\"\");";
                    println!("ouput_code updated; print (\\n)");
                }
            }
        }

        if token_code[1] == "var" {
            ouput_code_clone += "println!(v{}.as_str)";
            println!("ouput_code updated; print (var)");
            println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being not string.", iter.to_string(), line);
        }
    }

    return ouput_code_clone;
}

pub fn wait(token_code: Vec<String>, ouput_code: String, iter: i32, line: String) -> String {
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
        }
    }

    return ouput_code_clone;
}

pub fn variable(token_code: Vec<String>, ouput_code: String, iter: i32, line: String) -> String {
    let mut ouput_code_clone = ouput_code.clone();

    if token_code[0] == "var" {
        if token_code[2] == "int" {
            if token_code[1] == "not-exists" {
                ouput_code_clone += format!("let mut v{} = {};", token_code[3], token_code[4]).as_str();
                println!("ouput_code updated; var (not-exists)");
            }

            if token_code[1] == "exists" {
                ouput_code_clone += format!("v{} = {}", token_code[3], token_code[4]).as_str();
                println!("ouput_code updated; var (exists)");
                println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being wrong type", iter.to_string(), line);
            }
        }

        if token_code[2] == "string" {
            if token_code.len() > 5 {

                let mut var_string = String::from("");  // Current variable; finding value of string due to tokenazation spliting up string
                let mut iteration = 0;
                println!("Iterator line {} created", iter.to_string());

                for i in &token_code {
                    iteration += 1;

                    if iteration >= 5 {
                        var_string += " "; var_string += i.as_str();
                    }
                }

                if token_code[1] == "not-exists" {
                    ouput_code_clone += format!("let mut v{} = {};", token_code[3], token_code[4]).as_str();
                    println!("ouput_code updated; var (not-exists)");
                }
    
                if token_code[1] == "exists" {
                    ouput_code_clone += format!("v{} = {}", token_code[3], token_code[4]).as_str();
                    println!("ouput_code updated; var (exists)");
                    println!("Warning line {} :\n'{}' Code may not compile due to variable not existing or being wrong type", iter.to_string(), line);
                }
            }
        }
    }

    return ouput_code_clone;
}