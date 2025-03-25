use crate::candidate::CandidateBase;

#[test]
fn is_common_candidate_work_correctly() {
    let mut c1 = CandidateBase::new();
    let c2 = CandidateBase::from(("t", 1, 3, Some(5.0)));
    c1.set_type("r#type");
    println!("{:?}\n{:?}", c1, c2);
}
