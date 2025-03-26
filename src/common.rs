use std::sync::{Arc, Mutex};

pub type An<T> = Arc<Mutex<T>>;
