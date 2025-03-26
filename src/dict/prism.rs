use daachorse::DoubleArrayAhoCorasick;

use crate::algo::syllabifier::SyllableId;

use super::mapped_file::List;

type Credibility = f32;

struct SpellingDescriptor {
    syllable_id: SyllableId,
    r#type: i32,
    credibility: Credibility,
    tips: String,
}

type SpellingMapItem = List<SpellingDescriptor>;

struct Metadata;

pub trait ToleranceSearch {
    fn tolerance_search(prism: &Prism, key: &String, result: super::corrector::Corrections);
}
#[derive(Debug)]
pub struct Prism;

#[derive(Debug, Default)]
pub struct Match {
    value: usize,
    length: usize,
}

// impl Match {
//     fn result_pair() -> Self {

//     }
// }
