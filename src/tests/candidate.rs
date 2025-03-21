use crate::candidate::Candidate;

#[test]
fn is_common_candidate_work_correctly() {
    let mut c1 = Candidate::new();
    let c2 = Candidate::from("t", 1, 3, 5.0);
    c1.set_type("r#type");
    println!("{:?}\n{:?}", c1, c2);
}
