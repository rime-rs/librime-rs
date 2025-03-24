use std::{any::Any, cmp::Ordering, rc::Rc};

use fst::map::Values;
use petgraph::adj::List;

pub trait Candidates: Any {
    fn as_any(&self) -> &dyn Any; // 用于向下转换
    fn r#type(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn quality(&self) -> f64;
}
trait Optional {
    // candidate text to commit
    fn text(&self) -> &str;
    // (optional)
    fn comment(&self) -> String;
    // text shown in the preedit area, replacing input string (optional)
    fn preedit(&self) -> String;
}

#[derive(Default, Debug)]
pub struct Candidate {
    r#type: String,
    start: usize,
    end: usize,
    quality: f64,
}

impl Candidates for Candidate {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn r#type(&self) -> &str {
        &self.r#type
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn quality(&self) -> f64 {
        self.quality
    }
}

impl Candidate {
    pub fn new() -> Self {
        Candidate::default()
    }
    pub fn from(r#type: &str, start: usize, end: usize, quality: f64) -> Self {
        Self {
            r#type: String::from(r#type),
            start: start,
            end: end,
            quality: quality,
        }
    }

    pub fn get_genuine_candidate(cadn: &mut Rc<Candidate>) -> Vec<Rc<Candidate>> {
        let result = Vec::new();

        result
    }

    pub fn compare(&self, other: Self) -> i32 {
        let mut k: i32 = i32::try_from(self.start).unwrap() - i32::try_from(other.start).unwrap();
        // the one nearer to the beginning of segment comes first
        if k != 0 {
            return k.try_into().unwrap();
        }
        // then the longer comes first
        k = i32::try_from(self.end).unwrap() - i32::try_from(other.end).unwrap();
        if k != 0 {
            return -k;
        }
        // compare quality
        let qdiff: f64 = self.quality - other.quality;
        if qdiff != 0.0 {
            return if qdiff > 0.0 { -1 } else { 1 };
        }
        // draw
        0
    }

    // pub fn get_genuine_candidate() -> Rc<Candidate> {
    //     let uniquified =
    // }

    pub fn set_type(&mut self, r#type: &str) {
        self.r#type = String::from(r#type);
    }
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }
    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }
    pub fn quality(&mut self, quality: f64) {
        self.quality = quality;
    }
}

type CandidateQueue = List<Rc<Candidate>>;
type CandidateList = Vec<Rc<Candidate>>;

pub struct SimpleCandidate {
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) common: Candidate,
    pub(crate) preedit: String,
}

impl Optional for SimpleCandidate {
    fn text(&self) -> &str {
        &self.text
    }

    fn comment(&self) -> String {
        self.comment.clone()
    }

    fn preedit(&self) -> String {
        self.preedit.clone()
    }
}

impl SimpleCandidate {
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = String::from(comment);
    }
    pub fn set_preedit(&mut self, preedit: &str) {
        self.preedit = String::from(preedit);
    }
}

pub struct ShadowCandidate {
    pub(crate) common: Candidate,
    pub(crate) text: String,
    pub(crate) comment: String,
    // pub(crate) item: Box<dyn Candidate>,
    pub(crate) inherit_comment: bool,
}

pub struct UniquifiedCandidate {
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) item: CandidateList,
}
