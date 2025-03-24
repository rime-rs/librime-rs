pub fn contains_extended_cjk(text: &str) -> bool {
    text.chars().any(is_extended_cjk)
}

fn is_extended_cjk(ch: char) -> bool {
    let code_point = ch as u32;
    (0x3400..=0x4DBF).contains(&code_point) ||     // CJK Unified Ideographs Extension A
    (0x20000..=0x2A6DF).contains(&code_point) ||   // CJK Unified Ideographs Extension B
    (0x2A700..=0x2B73F).contains(&code_point) ||   // CJK Unified Ideographs Extension C
    (0x2B740..=0x2B81F).contains(&code_point) ||   // CJK Unified Ideographs Extension D
    (0x2B820..=0x2CEAF).contains(&code_point) ||   // CJK Unified Ideographs Extension E
    (0x2CEB0..=0x2EBEF).contains(&code_point) ||   // CJK Unified Ideographs Extension F
    (0x30000..=0x3134F).contains(&code_point) ||   // CJK Unified Ideographs Extension G
    (0x31350..=0x323AF).contains(&code_point) ||   // CJK Unified Ideographs Extension H
    (0x2EBF0..=0x2EE5D).contains(&code_point) ||   // CJK Unified Ideographs Extension I
    (0x3300..=0x33FF).contains(&code_point) ||     // CJK Compatibility
    (0xFE30..=0xFE4F).contains(&code_point) ||     // CJK Compatibility Forms
    (0xF900..=0xFAFF).contains(&code_point) ||     // CJK Compatibility Ideographs
    (0x2F800..=0x2FA1F).contains(&code_point) // CJK Compatibility Ideographs Supplement
}
