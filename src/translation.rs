use std::{any::Any, usize};

use petgraph::adj::List;

use crate::{
    candidate::{self, Candidate, CandidateList},
    common::{An, Of},
};

pub trait Translation: Any {
    /// A translation may contain multiple results, looks
    /// something like a generator of candidates.
    fn next(&self) -> bool;

    fn peek(&self) -> An<dyn Candidate>;

    fn exhausted(&self) -> bool {
        false
    }
}

/// should it provide the next candidate (negative value, zero) or
/// should it give up the chance for other translations (positive)?
pub fn compare(other: An<dyn Translation>, candidates: &CandidateList) -> i32 {
    todo!()
}

pub struct UniqueTranslation {
    exhausted: bool,
    pub(crate) candidate: Option<An<dyn Candidate>>,
}

impl Translation for UniqueTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> An<dyn Candidate> {
        todo!()
    }

    fn exhausted(&self) -> bool {
        self.exhausted
    }
}

impl UniqueTranslation {
    pub fn new(candidate: Option<An<dyn Candidate>>) -> Self {
        Self {
            exhausted: (candidate.is_none()),
            candidate: (candidate),
        }
    }
}

pub struct FifoTranslation {
    exhausted: bool,
    candies: CandidateList,
    cursor: usize,
}

impl Translation for FifoTranslation {
    fn exhausted(&self) -> bool {
        todo!()
    }

    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> An<dyn Candidate> {
        todo!()
    }
}

impl FifoTranslation {
    pub fn new() -> Self {
        todo!()
    }

    pub fn append(candy: An<dyn Candidate>) {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.candies.len() - self.cursor
    }
}

pub struct UnionTranslation {
    exhausted: bool,
    pub(crate) translations: List<Of<dyn Translation>>,
}

impl Translation for UnionTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> An<dyn Candidate> {
        todo!()
    }

    fn exhausted(&self) -> bool {
        todo!()
    }
}

impl UnionTranslation {
    pub fn new() -> Self {
        todo!()
    }
    pub fn add(&mut self, t: An<dyn Translation>) -> &Self {
        todo!()
    }
}

fn translation_union(x: An<dyn Translation>, y: An<dyn Translation>) -> An<UnionTranslation> {
    todo!()
}

struct MergeTranslation {
    previous_candidates: An<CandidateList>,
    translations: Vec<Of<dyn Translation>>,
    elected: usize,
}

impl Translation for MergeTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> An<dyn Candidate> {
        todo!()
    }

    fn exhausted(&self) -> bool {
        false
    }
}

impl MergeTranslation {
    pub fn new(previous_candidates: An<CandidateList>) -> Self {
        todo!()
    }

    pub fn add(&self, t: An<dyn Translation>) -> &Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.translations.len()
    }

    pub(crate) fn elect() {
        todo!()
    }
}
