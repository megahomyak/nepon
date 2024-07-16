use crate::parser::Program;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, program: Program) {
        println!("{program:?}");
    }
}
