use std::process;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line(pub Vec<String>);

#[must_use]
pub fn lex(code: String) -> Vec<Line> {
    if code.is_empty() {
        println!("[Error] Empty code.");
        println!("[Reason] Source code cannot be empty.");
        process::exit(1);
    }

    let mut lexed_code: Vec<Line> = Vec::new();
    let mut lexed_code_line: Vec<String> = Vec::new();
    let mut temp: String = String::new();
    let mut is_string: bool = false;
    let mut temp_string: String = String::new();

    for (line_number, line) in code
        .replace(['\r', '\t'], "")
        .trim()
        .split('\n')
        .enumerate()
    {
        let line_characters = &(*line).chars().collect::<Vec<char>>();
        for (character_index, &c) in line_characters.clone().iter().enumerate() {
            if c == '"' {
                if !is_string {
                    is_string = true
                } else {
                    is_string = false;
                    temp_string.push(c);
                }
            }

            if is_string && character_index == line_characters.len() - 1 {
                println!("[Error] String was never ended.");
                println!("[Code]\n{line} | {line_number}");
                println!(r#"[Help] Add the missing `"` at the end of the string."#);
                process::exit(1);
            } else if is_string {
                temp_string.push(c);
            } else {
                if !temp_string.is_empty() {
                    lexed_code_line.push(temp_string.clone());
                    temp_string = String::new();
                }

                if c == ' ' {
                    lexed_code_line.push(temp.clone());
                    temp = String::new();
                } else if character_index == line_characters.len() - 1 && c != '"' {
                    temp.push(c);
                    lexed_code_line.push(temp.clone());
                    temp = String::new();
                } else if !vec!['\"'].contains(&c) {
                    temp.push(c);
                }
            }
        }

        lexed_code_line.retain(|x| !(*x).is_empty());

        lexed_code.push(Line(lexed_code_line));
        lexed_code_line = Vec::new();
    }

    return lexed_code
}
