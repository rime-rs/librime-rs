use crate::common::An;
use std::any::Any;
use std::collections::LinkedList;

pub trait Candidate: Any {
    /// recognized by translators in learning phase
    fn r#type(&self) -> &str;
    /// [start, end) mark a range in the input that the candidate corresponds to
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn quality(&self) -> f64;

    /// candidate text to commit
    fn text(&self) -> String;
    /// (optional)
    fn comment(&self) -> String {
        String::new()
    }
    /// text shown in the preedit area, replacing input string (optional)
    fn preedit(&self) -> String {
        String::new()
    }

    fn set_type(&mut self, r#type: &str);
    fn set_start(&mut self, start: usize);
    fn set_end(&mut self, end: usize);
    fn set_quality(&mut self, quality: f64);

    fn as_any(&self) -> &dyn Any;
}

pub fn compare(selfs: An<dyn Candidate>, other: An<dyn Candidate>) -> i32 {
    let mut k: i32 = i32::try_from(selfs.lock().unwrap().start()).unwrap()
        - i32::try_from(other.lock().unwrap().start()).unwrap();
    // the one nearer to the beginning of segment comes first
    if k != 0 {
        return k.try_into().unwrap();
    }
    // then the longer comes first
    k = i32::try_from(selfs.lock().unwrap().end()).unwrap()
        - i32::try_from(other.lock().unwrap().end()).unwrap();
    if k != 0 {
        return -k;
    }
    // compare quality
    let qdiff: f64 = selfs.lock().unwrap().quality() - other.lock().unwrap().quality();
    if qdiff != 0.0 {
        return if qdiff > 0.0 { -1 } else { 1 };
    }
    // draw
    0
}

pub fn get_genuine_candidate(cand: &mut An<dyn Candidate>) -> An<dyn Candidate> {
    if let Some(uniquified) = cand
        .lock()
        .unwrap()
        .as_any()
        .downcast_ref::<UniquifiedCandidate>()
    {
        uniquified
            .items
            .first()
            .cloned()
            .unwrap_or_else(|| cand.clone())
    } else {
        cand.clone()
    }
}

pub fn get_genuine_candidates(cand: &mut An<dyn Candidate>) -> Vec<An<dyn Candidate>> {
    let mut result = Vec::new();
    if let Some(uniquified) = cand
        .lock()
        .unwrap()
        .as_any()
        .downcast_ref::<UniquifiedCandidate>()
    {
        for item in &uniquified.items {
            result.push(unpack_shadow_candidate(item));
        }
    } else {
        result.push(unpack_shadow_candidate(cand));
    }
    result
}

fn unpack_shadow_candidate(cand: &An<dyn Candidate>) -> An<dyn Candidate> {
    if let Some(shadow) = cand
        .lock()
        .unwrap()
        .as_any()
        .downcast_ref::<ShadowCandidate>()
    {
        shadow.item.clone()
    } else {
        cand.clone()
    }
}

#[derive(Default, Debug)]
pub struct CandidateBase {
    r#type: String,
    start: usize,
    end: usize,
    quality: f64,
}

impl From<(&str, usize, usize, Option<f64>)> for CandidateBase {
    fn from((r#type, start, end, quality): (&str, usize, usize, Option<f64>)) -> Self {
        Self {
            r#type: r#type.to_string(),
            start,
            end,
            quality: quality.unwrap_or_default(),
        }
    }
}

pub type CandidateQueue = LinkedList<An<CandidateBase>>;
pub type CandidateList = Vec<An<dyn Candidate>>;

// useful implementations

#[derive(Debug, Default)]
pub struct SimpleCandidate {
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) preedit: String,
    pub(crate) candidate: CandidateBase,
}

impl From<(&str, usize, usize, &str, Option<&str>, Option<&str>)> for SimpleCandidate {
    fn from(
        (r#type, start, end, text, comment, preedit): (
            &str,
            usize,
            usize,
            &str,
            Option<&str>,
            Option<&str>,
        ),
    ) -> Self {
        Self {
            text: text.to_string(),
            comment: comment.unwrap_or_default().to_string(),
            preedit: preedit.unwrap_or_default().to_string(),
            candidate: CandidateBase::from((r#type, start, end, None)),
        }
    }
}

impl Candidate for SimpleCandidate {
    fn r#type(&self) -> &str {
        &self.candidate.r#type
    }
    fn start(&self) -> usize {
        self.candidate.start
    }
    fn end(&self) -> usize {
        self.candidate.end
    }

    fn quality(&self) -> f64 {
        self.candidate.quality
    }

    fn text(&self) -> String {
        self.text.to_string()
    }

    fn comment(&self) -> String {
        self.comment.to_string()
    }

    fn preedit(&self) -> String {
        self.preedit.to_string()
    }

    fn set_type(&mut self, r#type: &str) {
        self.candidate.r#type = String::from(r#type);
    }
    fn set_start(&mut self, start: usize) {
        self.candidate.start = start;
    }
    fn set_end(&mut self, end: usize) {
        self.candidate.end = end;
    }
    fn set_quality(&mut self, quality: f64) {
        self.candidate.quality = quality;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SimpleCandidate {
    pub fn new() -> Self {
        Self::default()
    }

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
    pub(crate) candidate: CandidateBase,
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) item: An<dyn Candidate>,
    pub(crate) inherit_comment: bool,
}

impl Candidate for ShadowCandidate {
    fn r#type(&self) -> &str {
        &self.candidate.r#type
    }

    fn start(&self) -> usize {
        self.candidate.start
    }

    fn end(&self) -> usize {
        self.candidate.end
    }

    fn quality(&self) -> f64 {
        self.candidate.quality
    }

    fn text(&self) -> String {
        if self.text.is_empty() {
            self.item.lock().unwrap().text().to_string()
        } else {
            self.text.to_string()
        }
    }

    fn comment(&self) -> String {
        if self.inherit_comment && self.comment.is_empty() {
            self.item.lock().unwrap().comment()
        } else {
            self.comment.clone()
        }
    }

    fn preedit(&self) -> String {
        self.item.lock().unwrap().preedit()
    }

    fn set_type(&mut self, r#type: &str) {
        self.candidate.r#type = String::from(r#type);
    }
    fn set_start(&mut self, start: usize) {
        self.candidate.start = start;
    }
    fn set_end(&mut self, end: usize) {
        self.candidate.end = end;
    }
    fn set_quality(&mut self, quality: f64) {
        self.candidate.quality = quality;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl
    From<(
        An<dyn Candidate>,
        &str,
        Option<&str>,
        Option<&str>,
        Option<bool>,
    )> for ShadowCandidate
{
    fn from(
        (item, r#type, text, comment, inherit_comment): (
            An<dyn Candidate>,
            &str,
            Option<&str>,
            Option<&str>,
            Option<bool>,
        ),
    ) -> Self {
        Self {
            candidate: CandidateBase::from((
                r#type,
                item.try_lock().as_ref().unwrap().start(),
                item.try_lock().as_ref().unwrap().end(),
                Some(item.try_lock().as_ref().unwrap().quality()),
            )),
            text: text.unwrap_or_default().to_string(),
            comment: comment.unwrap_or_default().to_string(),
            item: item.clone(),
            inherit_comment: inherit_comment.unwrap_or_default(),
        }
    }
}

#[derive(Default)]
pub struct UniquifiedCandidate {
    pub(crate) candidate: CandidateBase,
    pub(crate) text: String,
    pub(crate) comment: String,
    pub(crate) items: CandidateList,
}

impl Candidate for UniquifiedCandidate {
    fn r#type(&self) -> &str {
        &self.candidate.r#type
    }

    fn start(&self) -> usize {
        self.candidate.start
    }

    fn end(&self) -> usize {
        self.candidate.end
    }

    fn quality(&self) -> f64 {
        self.candidate.quality
    }

    fn text(&self) -> String {
        match (self.text.is_empty(), self.items.first()) {
            (true, Some(item)) => item.lock().unwrap().text(),
            _ => self.text.to_string(),
        }
    }

    fn comment(&self) -> String {
        match (self.comment.is_empty(), self.items.first()) {
            (true, Some(item)) => item.lock().unwrap().comment(),
            _ => self.comment.clone(),
        }
    }

    fn preedit(&self) -> String {
        self.items
            .first()
            .map_or_else(String::new, |item| item.lock().unwrap().preedit())
    }

    fn set_type(&mut self, r#type: &str) {
        self.candidate.r#type = String::from(r#type);
    }
    fn set_start(&mut self, start: usize) {
        self.candidate.start = start;
    }
    fn set_end(&mut self, end: usize) {
        self.candidate.end = end;
    }
    fn set_quality(&mut self, quality: f64) {
        self.candidate.quality = quality;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl UniquifiedCandidate {
    pub fn append(&mut self, item: An<dyn Candidate>) {
        self.items.push(item.clone());
        if self.quality() < item.lock().unwrap().quality() {
            self.set_quality(item.lock().unwrap().quality());
        }
    }
}
