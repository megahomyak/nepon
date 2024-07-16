use std::{collections::HashMap, rc::Rc};

use crate::{objects, parser::Program};
use downcast_rs::{impl_downcast, Downcast};

pub trait Object: Downcast {
    fn to_string(&self) -> String;
}
impl_downcast!(Object);

pub struct Scope(pub HashMap<String, Rc<dyn Object>>);

pub struct Interpreter {
    pub names: Scope,
}

impl Interpreter {
    pub fn interpret(&mut self, program: Program) {
        // TODO: GET RID OF EVERYTHING WITH "TODO", REPLACE IT WITH ERRORS!!!
        for line in program.lines {
            let mut object = self
                .names
                .0
                .get_mut(&line.command_name)
                .expect("object does not exist TODO")
                .clone();
            for input in line.inputs {
                let command: Rc<objects::Command> = object
                    .downcast_rc()
                    .unwrap_or_else(|_| panic!("not a command TODO"));
                object = command.0(input);
            }
            if object.downcast_ref::<objects::Nothing>().is_none() {
                println!("{}", object.to_string());
            }
        }
    }
}
