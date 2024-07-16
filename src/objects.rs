use std::rc::Rc;

use crate::interpreter::Object;

pub struct Command(pub Box<dyn Fn(String) -> Rc<dyn Object>>);
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
