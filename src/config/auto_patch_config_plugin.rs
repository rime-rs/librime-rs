pub fn remove_suffix(input: &str, suffix: &str) -> String {
    if input.ends_with(suffix) {
        let end = input.len() - suffix.len();
        input[..end].to_string()
    } else {
        input.to_string()
    }
}
