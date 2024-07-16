mod parser;
mod interpreter;

fn main() {
    let mut editor = rustyline::DefaultEditor::new().unwrap();
    let mut interpreter = interpreter::Interpreter {};
    while let Ok(line) = editor.readline("> ") {
        editor.add_history_entry(&line).unwrap();
        match parser::program(&line) {
            Ok(program) => interpreter.interpret(program),
            Err(e) => println!("{e:?}"),
        }
    }
}
