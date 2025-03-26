use std::sync::{Arc, Mutex};

use crate::candidate::{Candidate, SimpleCandidate, compare};

#[test]
fn is_common_candidate_work_correctly() {
    let mut c1 = SimpleCandidate::new();
    let c2 = SimpleCandidate::from(("t", 1 as usize, 3 as usize, "", Some("fku"), Some("2244")));
    c1.set_type("r#type");
    println!("\n{:?}\n{:?}", c1, c2);
    let c1p = Arc::new(Mutex::new(c1));
    let c2p = Arc::new(Mutex::new(c2));
    println!("\n{:?}", compare(c1p, c2p));
}
