use std::rc::Rc;

use crate::interpreter::{Interpreter, Object};

pub struct Command(pub Box<dyn Fn(String, &mut Interpreter) -> Rc<dyn Object>>);
impl Object for Command {
    fn to_string(&self) -> String {
        "command".to_owned()
    }
}

pub struct Nothing {}
impl Object for Nothing {
    fn to_string(&self) -> String {
        "nothing".to_owned()
    }
}

pub struct Error(pub String);
impl Object for Error {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
