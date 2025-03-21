use std::path::Path;

use crate::algo::utilities::{checksum, compare_version_string};

#[test]
fn is_compare_version_string_correctly() {
    assert_eq!(compare_version_string("1", "1"), 0);
    assert_eq!(compare_version_string("1", "0"), 1);
    assert_eq!(compare_version_string("0", "1"), -1);
    assert_eq!(compare_version_string("1.01", "1.001"), 0);
    assert_eq!(compare_version_string("1", "1.0.0"), 0);
    assert_eq!(compare_version_string("123", "3"), 1);
    assert_eq!(compare_version_string("1.10.1.rs", "1.10.1"), 0);
    assert_eq!(compare_version_string("1.10.1rs", "1.10.1"), 0);
    assert_eq!(compare_version_string("1.10.1-rs", "1.10c.1"), 0);
    assert_eq!(compare_version_string("2024-02-05", "0.40"), 1);
}

#[test]
fn is_check_crc_sum_correctly() {
    println!("crcsum: {} - LICENSE", checksum(Path::new("LICENSE")));
    println!("crcsum: {} - .gitignore", checksum(Path::new(".gitignore")));
    println!("crcsum: {} - README.md", checksum(Path::new("README.md")));
}
