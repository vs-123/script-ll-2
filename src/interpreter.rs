use std::{collections::HashMap, process};

use regex::Regex;

use crate::ast::{self, get_code_from, has_label, Label};

pub struct Interpreter {
    labels: Vec<Label>,
    variables: HashMap<String, String>,
    current_line_number: usize,
    current_line_code: String,
}

fn get_type(token: String) -> ast::Types {
    let number_re = Regex::new(r"^[0-9]+(\.[0-9]+)?$").unwrap();
    let identifier_re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

    if token.starts_with('\"') && token.ends_with('\"') {
        return ast::Types::String
    } else if number_re.is_match(&token) {
        return ast::Types::Number;
    } else if identifier_re.is_match(&token) {
        return ast::Types::Identifier;
    } else {
        return ast::Types::Unknown;
    }
}

// fn string_to_type(string: String) -> ast::Types {
//     match string.as_str() {
//         "Number" => ast::Types::Number,

//         "String" => ast::Types::String,

//         _ => todo!(),
//     }
// }

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    return chars.as_str();
}

fn get_string_content(string: String) -> String {
    if string.starts_with('\"') && string.ends_with('\"') {
        return rem_first_and_last(&string).to_string().replace("\\n", "\n");
    } else {
        return string
    }
}

impl Interpreter {
    #[must_use]
    pub fn new(labels: Vec<Label>) -> Self {
        return Self {
            labels,
            variables: HashMap::new(),
            current_line_code: String::new(),
            current_line_number: 0,
        }
    }

    fn get_variable(&self, variable_name: String) -> (String, ast::Types) {
        match self.variables.clone().get(&variable_name) {
            Some(value) => {
                let value = (*value).to_string();
                let value_type = get_type(value.clone());
                if value_type == ast::Types::Identifier {
                    let value = self.get_variable(value);
                    return (value.0, value.1)
                } else if value_type == ast::Types::String {
                    return (get_string_content(value), value_type);
                } else {
                    return (value, value_type);
                }
            }

            None => {
                println!("[Error] Variable `{variable_name}` does not exist.");
                println!("[Code]");
                println!("{} | {}", self.current_line_number, self.current_line_code);
                process::exit(1);
            }
        }
    }

    pub fn interpret_label(&mut self, label_name: String) {
        for (line_number, line) in get_code_from(self.labels.clone(), label_name.clone()) {
            let command = line.0[0].clone();
            let command = command.as_str();
            self.current_line_number = line_number + 1;
            self.current_line_code = line.0.join(" ").clone();

            let arguments = line.0[1..].to_vec();
            let _no_of_args = arguments.len();

            self.interpret_command(command, arguments, label_name.clone());
        }
    }

    pub fn interpret_command(&mut self, command: &str, arguments: Vec<String>, label_name: String) {
        let no_of_args = arguments.len();
        match command {
            // Comment
            "cmt" => {
                // Ignore comments
            }

            "var" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] var <variable_name> <variable_value>");
                    process::exit(1);
                }

                let variable_name = arguments[0].clone();
                let variable_value = arguments[1].clone();

                self.variables.insert(variable_name, variable_value);
            }

            "jmp" => {
                if no_of_args != 1 {
                    println!("[Error] Expected exactly 1 argument, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] jmp <label_name>");
                    process::exit(1);
                }

                if has_label(self.labels.clone(), arguments[0].clone()) {
                    self.interpret_label(arguments[0].clone());
                } else {
                    println!("[Error] Label `{}` does not exist.", arguments[0].clone());
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Note] The label needs to exist");
                    process::exit(1);
                }
            }

            "require" => {
                if no_of_args != 1 {
                    println!("[Error] Expected exactly 1 argument, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] require <variable_name>");
                    process::exit(1);
                }

                if self.variables.clone().get(&arguments[0]).is_none() {
                    println!(
                        "[Error] Variable `{}` does not exist, but is required in label `{}`",
                        arguments[0], label_name
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    process::exit(1);
                }
            }

            // Test if >=
            "test_gt_eq" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt_eq <number1> <number2>");
                    process::exit(1);
                }

                let n1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let n2 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                if get_type(n1.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n1)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt_eq <number1> <number2>");
                    process::exit(1);
                }

                if get_type(n2.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n2)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt_eq <number1> <number2>");
                    process::exit(1);
                }

                self.variables.insert(String::from("TEMP"), {
                    if n1.parse::<f32>().unwrap() >= n2.parse::<f32>().unwrap() {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
            }

            "test_eq" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_eq <value1> <value2>");
                    process::exit(1);
                }

                let n1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let n2 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                self.variables.insert(String::from("TEMP"), {
                    if n1 == n2 {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
            }

            "test_lt_eq" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt_eq <number1> <number2>");
                    process::exit(1);
                }

                let n1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let n2 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                if get_type(n1.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n1)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_lt_eq <number1> <number2>");
                    process::exit(1);
                }

                if get_type(n2.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n2)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_lt_eq <number1> <number2>");
                    process::exit(1);
                }

                self.variables.insert(String::from("TEMP"), {
                    if n1.parse::<f32>().unwrap() <= n2.parse::<f32>().unwrap() {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
            }

            "test_gt" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt <number1> <number2>");
                    process::exit(1);
                }

                let n1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let n2 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                if get_type(n1.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n1)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt <number1> <number2>");
                    process::exit(1);
                }

                if get_type(n2.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n2)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_gt <number1> <number2>");
                    process::exit(1);
                }

                self.variables.insert(String::from("TEMP"), {
                    if n1.parse::<f32>().unwrap() > n2.parse::<f32>().unwrap() {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
            }

            "test_lt" => {
                if no_of_args != 2 {
                    println!("[Error] Expected exactly 2 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_lt <number1> <number2>");
                    process::exit(1);
                }

                let n1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let n2 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                if get_type(n1.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n1)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_lt <number1> <number2>");
                    process::exit(1);
                }

                if get_type(n2.clone()) != ast::Types::Number {
                    println!(
                        "[Error] Expected the first value to be a Number, not a {}",
                        get_type(n2)
                    );
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] test_lt <number1> <number2>");
                    process::exit(1);
                }

                self.variables.insert(String::from("TEMP"), {
                    if n1.parse::<f32>().unwrap() < n2.parse::<f32>().unwrap() {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
            }

            "cmd_eq" => {
                if no_of_args < 4 {
                    println!("[Error] Expected at least 4 arguments, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] cmd_eq <value1> <value2> <command> <args>...");
                    process::exit(1);
                }

                let x1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                let x2 = {
                    if get_type(arguments[1].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[1].clone()
                    }
                };

                if x1 == x2 {
                    self.interpret_command(&arguments[2], arguments[3..].to_vec(), label_name)
                }
            }

            "print" => {
                if no_of_args != 1 {
                    println!("[Error] Expected exactly 1 argument, found {no_of_args}");
                    println!("[Code]");
                    println!("{} | {}", self.current_line_number, self.current_line_code);
                    println!("[Usage] print <value>");
                    process::exit(1);
                }

                let x1 = {
                    if get_type(arguments[0].clone()) == ast::Types::Identifier {
                        self.get_variable(arguments[0].clone()).0
                    } else {
                        arguments[0].clone()
                    }
                };

                println!("{}", get_string_content(x1));
            }

            _ => {
                println!("[Error] Unknown command `{command}`");
                println!("[Code]");
                println!("{} | {}", self.current_line_number, self.current_line_code);
                process::exit(1);
            }
        }
    }

    pub fn interpret(&mut self) {
        self.variables.insert(String::from("TEMP"), String::new());
        self.interpret_label(".ENTRY".to_string());
    }
}
