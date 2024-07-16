use std::{collections::HashMap, rc::Rc};

mod interpreter;
mod parser;
mod objects;

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
    while let Ok(line) = editor.readline("> ") {
        editor.add_history_entry(&line).unwrap();
        match parser::program(&line) {
            Ok(program) => interpreter.interpret(program),
            Err(e) => println!("{e:?}"),
        }
    }
}
