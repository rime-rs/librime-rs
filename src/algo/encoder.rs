trait PhraseCollector {
    fn create_entry(&mut self, phrase: &str, code_str: &str, value: &str);
    // return a list of alternative code for the given word
    fn translate_word(&self, word: &str) -> Option<Vec<String>>;
}
