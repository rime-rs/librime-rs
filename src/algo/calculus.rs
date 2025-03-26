use super::spelling::Spelling;
use std::{collections::HashMap, rc::Rc, vec::Vec};

const ABBREVIATION_PENALTY: f64 = -0.6931471805599453; // log(0.5)
const FUZZY_SPELLING_PENALTY: f64 = -0.6931471805599453; // log(0.5)

pub(crate) trait Calculation {
    // type Factory: Fn(&Vec<String>) -> Box<dyn Calculation>;
    fn apply(&mut self, spelling: &mut Spelling) -> bool;
    fn addition(&self) -> bool {
        true
    }
    fn deletion(&self) -> bool {
        true
    }
}

pub struct DefaultCalculation;

pub struct Calculus {
    factories: HashMap<String, Rc<dyn Fn(Vec<String>) -> Option<Box<dyn Calculation>>>>,
}

impl Calculus {
    pub fn calculus() {}
    pub(crate) fn register(token: &str) {}
    pub fn parse() -> Box<DefaultCalculation> {
        return Box::new(DefaultCalculation);
    }
}
