use std::{collections::HashMap, rc::Rc};

use crate::{
    objects,
    parser::{Program, S},
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

impl Interpreter {
    pub fn interpret<'a>(&mut self, program: Program<'a>) -> Result<Rc<dyn Object>, Error<'a>> {
        let mut last_obj: Rc<dyn Object> = Rc::new(objects::Nothing {});
        for line in program.lines {
            last_obj = self
                .names
                .0
                .get_mut(&line.command_name.content)
                .ok_or(Error::UnknownName {
                    beginning_pos: line.command_name.beginning_pos,
                })?
                .clone();
            for input in line.inputs {
                let command: Rc<objects::Command> =
                    last_obj
                        .downcast_rc()
                        .map_err(|_| Error::CallingANonCommand {
                            opening_paren_pos: input.opening_paren_pos,
                        })?;
                last_obj = command.0(input.content);
            }
        }
        Ok(last_obj)
    }
}
