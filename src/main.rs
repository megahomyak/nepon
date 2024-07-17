use std::{collections::HashMap, rc::Rc};

mod interpreter;
mod objects;
mod parser;

fn main() {
    fn populate_fn(
        scope: &mut interpreter::Scope,
        n: &str,
        o: impl Fn(String) -> Rc<dyn interpreter::Object> + 'static,
    ) {
        scope
            .0
            .insert(n.to_owned(), Rc::new(objects::Command(Box::new(o))));
    }

    let mut names = interpreter::Scope(HashMap::new());
    populate_fn(&mut names, "print", |input| {
        println!("{}", input);
        Rc::new(objects::Nothing {})
    });
    let mut interpreter = interpreter::Interpreter { names };

    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let mut buffer: String = String::new();
    while let Ok(line) = editor.readline(if buffer.is_empty() { "> " } else { "| " }) {
        editor.add_history_entry(&line).unwrap();
        buffer.push_str(&line);
        match parser::program((&buffer[..]).into()) {
            Ok(program) => {
                let obj = interpreter.interpret(program);
                if obj.downcast_ref::<objects::Nothing>().is_none() {
                    println!("{}", obj.to_string());
                }
            }
            Err(e) => {
                use parser::Error::*;
                let (s, reason) = match e {
                    MissingClosingParen { .. } => {
                        buffer.push('\n');
                        continue;
                    }
                    UnexpectedClosingParen { closing_paren_pos } => {
                        (closing_paren_pos, "Unexpected closing parenthesis")
                    }
                    InputWithoutCommandName { opening_paren_pos } => {
                        (opening_paren_pos, "Input without command name")
                    }
                    NothingAfterEscapeCharacter { esc_char_pos } => {
                        (esc_char_pos, "Nothing after escape character")
                    }
                };
                let (row, col, line) = parser::row_col_line(&s);
                eprintln!("Syntax error at row {row}, column {col}: {reason}");
                eprintln!("\n{line}");
                for _ in 0..col {
                    eprint!(" ");
                }
                eprintln!("^\n");
            }
        }
        buffer.clear();
    }
}
