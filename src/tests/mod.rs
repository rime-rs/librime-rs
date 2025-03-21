use super::*;

#[cfg(test)]
mod algo;
mod candidate;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
