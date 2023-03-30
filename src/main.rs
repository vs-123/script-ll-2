#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use ast::{has_label, Label};

use std::collections::HashMap;
use std::env::args;

use std::fs::{self};

use std::process::{self};

pub mod ast;
pub mod interpreter;
pub mod lexer;

use interpreter::Interpreter;

fn main() {
    let mut arguments = args();
    arguments.next().unwrap();

    match arguments.next() {
        Some(input_file) => {
            if input_file == "--h" {
                println!("[Help]");
                println!("label <label_name>                              Creates a label");
                println!("jmp <label_name>                                Jumps to a label");
                println!("var <name> <value>                              Makes a variable (Note: Variables are global and are not limited to a label)");
                println!("require <variable_name>                         Makes it necessary for variable <variable_name> to exist.");
                println!("test_lt_eq <number1> <number2>                  Tests whether <number1> is less than or equal to <number2>");
                println!("test_gt_eq <number1> <number2>                  Tests whether <number1> is greater than or equal to <number2>");
                println!("test_lt <number1> <number2>                     Tests whether <number1> is less than <number2>");
                println!("test_gt <number1> <number2>                     Tests whether <number1> is greater than <number2>");
                println!("test_eq <value1> <value2>                       Tests whether <value1> is equal to <value2>");
                println!("cmd_eq <value1> <value2> <command> <args>...    Executes <command> with arguments <args> if <value1> is equal to <value2>");
                println!("cmt <anything>...                               A comment. Ignored by the interpreter");
                println!();
                println!("[Basic Hello World script]");
                println!("label .ENTRY");
                println!("    print \"Hello World\"");
                return;
            }
            match fs::read_to_string(input_file.clone()) {
                Ok(code) => {
                    let lexed_code = lexer::lex(code);

                    let _variables: HashMap<String, String> = HashMap::new();
                    let mut labels: Vec<Label> = vec![];
                    let mut current_label: String = String::new();
                    let mut label_code: Vec<(usize, lexer::Line)> = Vec::new();

                    for (line_number, line) in lexed_code.iter().enumerate() {
                        let line_number = line_number + 1;
                        let line: Vec<String> = line.clone().0;
                        if line.is_empty() {
                            continue;
                        }
                        let string_line = line.clone().join(" ");

                        let command: String = line[0].clone();
                        let args: Vec<String> = line[1..].to_vec().clone();
                        let args_len = args.len();

                        match command.clone().as_str() {
                            "label" => {
                                if args_len != 1 {
                                    println!("[Error] Expected 1 argument, but got {args_len}");
                                    println!("[Code]");
                                    println!("{line_number} | {string_line}");
                                    println!("[Usage] label <label_name>");
                                    process::exit(1);
                                }

                                labels.push(Label {
                                    label_name: current_label.clone(),
                                    label_code,
                                });
                                label_code = Vec::new();

                                let label_name = args[0].clone();
                                current_label = label_name.clone();

                                if has_label(labels.clone(), label_name.clone()) {
                                    println!("[Error] Label `{label_name}` already exists.");
                                    println!("[Code]");
                                    println!("{line_number} | {string_line}");
                                    println!("[Help] Do not use an existing label name.");
                                    process::exit(1);
                                }
                            }

                            _ => {
                                label_code.push((line_number, lexer::Line(line.clone())));
                            }
                        }

                        if line_number == lexed_code.len() {
                            labels.push(Label {
                                label_name: current_label.clone(),
                                label_code,
                            });
                            label_code = Vec::new();
                        }
                    }

                    if has_label(labels.clone(), ".ENTRY".to_string()) {
                        let mut interpreter = Interpreter::new(labels);
                        interpreter.interpret();
                    } else {
                        println!("[Error] Label `.ENTRY` does not exist. (.ENTRY is the main entry point of the script.");
                        println!("[Help] Add a label named `.ENTRY` using `label .ENTRY`");
                        process::exit(1);
                    }
                }

                Err(e) => {
                    println!("[Error] Could not open file `{input_file}`");
                    println!("[Reason] {e}");
                    process::exit(1);
                }
            }
        }

        None => {
            let program: Vec<String> = args().collect();
            println!("[Usage] {} <source_code_file>", program[0]);
            println!("[Example] {} examples/tutorial.ll", program[0]);
            println!("[For help regarding the language] {} --h", program[0]);
        }
    }
}
