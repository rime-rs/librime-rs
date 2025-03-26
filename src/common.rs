use std::rc::Rc;

pub type The<T> = Box<T>;
pub type An<T> = Rc<T>;
pub type Of<T> = An<T>;
pub type Weak<T> = std::rc::Weak<T>;
