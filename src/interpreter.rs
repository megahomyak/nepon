use std::{collections::HashMap, rc::Rc};

use crate::{
    objects,
    parser::{self, Program, S},
};
use downcast_rs::{impl_downcast, Downcast};

pub trait Object: Downcast {
    fn to_string(&self) -> String;
}
impl_downcast!(Object);

pub struct Scope(pub HashMap<String, Rc<dyn Object>>);

pub struct Interpreter {
    pub names: Scope,
}

pub enum Error<'a> {
    UnknownName { beginning_pos: S<'a> },
    CallingANonCommand { opening_paren_pos: S<'a> },
}

impl Error<'_> {
    fn to_string(&self) -> String {
        let mut output = String::new();
        let (s, reason) = match self {
            Self::CallingANonCommand { opening_paren_pos } => {
                (opening_paren_pos, "Calling a non-command")
            }
            Self::UnknownName { beginning_pos } => (beginning_pos, "Unknown name"),
        };
        let (row, col, line) = parser::row_col_line(s);
        output.push_str(&format!(
            "Execution error at row {row}, column {col}: {reason}\n\n"
        ));
        output.push_str(line);
        output.push('\n');
        for _ in 1..col {
            output.push(' ');
        }
        output.push_str("^\n");
        output
    }
}

fn err(e: Error) -> Rc<dyn Object> {
    Rc::new(objects::Error(e.to_string()))
}

impl Interpreter {
    pub fn interpret<'a>(&mut self, program: Program<'a>) -> Rc<dyn Object> {
        let mut last_obj: Rc<dyn Object> = Rc::new(objects::Nothing {});
        for line in program.lines {
            last_obj = match self.names.0.get_mut(&line.command_name.content) {
                Some(last_obj) => last_obj,
                None => {
                    return err(Error::UnknownName {
                        beginning_pos: line.command_name.beginning_pos,
                    })
                }
            }
            .clone();
            for input in line.inputs {
                let command: &objects::Command = match last_obj.downcast_ref() {
                    Some(command) => command,
                    None => {
                        return err(Error::CallingANonCommand {
                            opening_paren_pos: input.opening_paren_pos,
                        })
                    }
                };
                last_obj = command.0(input.content, self);
            }
        }
        last_obj
    }
}
