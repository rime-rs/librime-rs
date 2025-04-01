use std::sync::{Arc, Mutex};

pub type The<T> = Box<T>;
pub type An<T> = Arc<Mutex<T>>;
pub type Of<T> = An<T>;
pub type Weak<T> = std::rc::Weak<T>;

