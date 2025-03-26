#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SpellingType {
    NormalSpelling,
    FuzzySpelling,
    Abbreviation,
    Completion,
    AmbiguousSpelling,
    InvalidSpelling,
}

#[derive(Debug, Clone)]
pub(crate) struct SpellingProperties {
    pub spelling_type: SpellingType,
    pub end_pos: usize,
    pub credibility: f64,
    pub tips: String,
}

impl Default for SpellingProperties {
    fn default() -> Self {
        Self {
            spelling_type: SpellingType::NormalSpelling,
            end_pos: 0,
            credibility: 0.0,
            tips: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Spelling {
    str: String,
    properties: SpellingProperties,
}

impl Spelling {
    pub fn new() -> Self {
        Self {
            str: String::new(),
            properties: SpellingProperties::default(),
        }
    }
    pub fn from(str: String) -> Self {
        Self {
            str,
            properties: SpellingProperties::default(),
        }
    }
}

impl PartialEq for Spelling {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl Eq for Spelling {}

impl PartialOrd for Spelling {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.str.partial_cmp(&other.str)
    }
}

impl Ord for Spelling {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.str.cmp(&other.str)
    }
}
