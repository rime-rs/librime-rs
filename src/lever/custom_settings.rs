pub fn remove_suffix(input: &str, suffix: &str) -> String {
    if input.ends_with(suffix) {
        let end = input.len() - suffix.len();
        input[..end].to_string()
    } else {
        input.to_string()
    }
}

pub fn custom_config_file(config_id: &str) -> String {
    format!("{}.custom.yaml", remove_suffix(config_id, ".schema"))
}
