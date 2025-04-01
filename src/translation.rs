use crate::{
    candidate::{self, Candidate, CandidateList},
    common::{An, Of},
};
use fst::raw::Transition;
use log::info;
use std::collections::{BTreeSet, LinkedList};
use std::env::Args;
use std::{any::Any, result, usize};
use tokio::task::id;

pub trait Translation: Any {
    /// A translation may contain multiple results, looks
    /// something like a generator of candidates.
    fn next(&mut self) -> bool;

    fn peek(&self) -> Option<An<dyn Candidate>>;

    /// should it provide the next candidate (negative value, zero) or
    /// should it give up the chance for other translations (positive)?
    fn compare(&self, other: Option<An<dyn Translation>>, candidates: &CandidateList) -> i32 {
        if other.as_ref().map_or(true, |t| t.is_exhausted()) {
            return -1;
        }
        if self.is_exhausted() {
            return 1;
        }
        let ours = self.peek();
        let theirs = other.as_ref().unwrap().peek();
        if ours.is_none() || theirs.is_none() {
            return 1;
        }
        ours.unwrap().compare(theirs.unwrap())
    }
    fn is_exhausted(&self) -> bool;

    fn set_exhausted(&mut self, exhausted: bool);
}

pub struct UniqueTranslation {
    exhausted: bool,
    pub(crate) candidate: Option<An<dyn Candidate>>,
}

impl Translation for UniqueTranslation {
    fn next(&mut self) -> bool {
        if self.is_exhausted() {
            return false;
        };
        self.set_exhausted(true);
        true
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        self.candidate.clone()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl UniqueTranslation {
    pub fn new(candidate: Option<An<dyn Candidate>>) -> Self {
        Self {
            exhausted: candidate.is_none(),
            candidate,
        }
    }
}

#[derive(Default)]
pub struct FifoTranslation {
    exhausted: bool,
    candies: CandidateList,
    cursor: usize,
}

impl Translation for FifoTranslation {
    fn next(&mut self) -> bool {
        if self.is_exhausted() {
            return false;
        };
        self.cursor += 1;
        if self.cursor > self.candies.len() {
            self.set_exhausted(true);
        };
        true
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        if self.is_exhausted() {
            return None;
        };
        self.candies.get(self.cursor).cloned()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl FifoTranslation {
    pub fn new() -> Self {
        let mut result = Self::default();
        result.set_exhausted(false);
        result
    }

    pub fn append(&mut self, candy: An<dyn Candidate>) {
        self.candies.push(candy);
        self.set_exhausted(false);
    }

    pub fn len(&self) -> usize {
        self.candies.len() - self.cursor
    }
}

#[derive(Default)]
pub struct UnionTranslation {
    exhausted: bool,
    pub(crate) translations: LinkedList<Of<dyn Translation>>,
}

impl Translation for UnionTranslation {
    fn next(&mut self) -> bool {
        if self.is_exhausted() {
            return false;
        };
        self.translations.front().unwrap().next();
        if self.translations.front().unwrap().is_exhausted() {
            self.translations.pop_front();
            if self.translations.is_empty() {
                self.set_exhausted(true)
            };
        };
        true
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        if self.is_exhausted() {
            return None;
        };
        self.translations.front().unwrap().peek()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl UnionTranslation {
    pub fn new() -> Self {
        let mut result = Self::default();
        result.set_exhausted(true);
        result
    }
    pub fn add(&mut self, t: Option<An<dyn Translation>>) -> &Self {
        if let Some(t) = t {
            if !t.is_exhausted() {
                self.translations.push_back(t);
                self.set_exhausted(false);
            }
        }
        self
    }
}

fn translation_union(
    x: An<dyn Translation>,
    y: An<dyn Translation>,
) -> Option<An<UnionTranslation>> {
    let mut z = An::new(UnionTranslation::new());
    z.add(Some(x));
    z.add(Some(y));
    if z.is_exhausted() { None } else { Some(z) }
}

#[derive(Default)]
struct MergedTranslation {
    exhausted: bool,
    previous_candidates: An<CandidateList>,
    translations: Vec<Of<dyn Translation>>,
    elected: usize,
}

impl Translation for MergedTranslation {
    fn next(&mut self) -> bool {
        if self.is_exhausted() {
            false
        } else {
            self.translations.get(self.elected).unwrap().next();
            if self.translations.get(self.elected).unwrap().is_exhausted() {
                info!("translation #{} has been exhausted.", self.elected);
                let mut tail = self.translations.split_off(self.elected);
                tail.pop_front();
                self.translations.append(&mut tail);
            }
            self.elect();
            !self.is_exhausted()
        }
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        if self.is_exhausted() {
            None
        } else {
            self.translations.get(self.elected).unwrap().peek()
        }
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl MergedTranslation {
    pub fn new(candidates: An<CandidateList>) -> Self {
        let mut result = Self::default();
        result.previous_candidates = candidates;
        result.set_exhausted(true);
        result
    }

    pub fn add(&mut self, t: Option<An<dyn Translation>>) -> &Self {
        if let Some(t) = t {
            if !t.is_exhausted() {
                self.translations.push(t);
                self.elect();
            }
        }
        self
    }

    pub fn len(&self) -> usize {
        self.translations.len()
    }

    pub(crate) fn elect(&mut self) {
        if self.translations.is_empty() {
            self.set_exhausted(true);
            return;
        }

        for k in 0..self.translations.len() {
            let current = self.translations.get_mut(k);
            let next = if k + 1 < self.translations.len() {
                Some(self.translations.get(k + 1))
            } else {
                None
            };
            // compare
        }
        todo!()
    }
}

struct CacheTranslation {
    exhausted: bool,

    pub(crate) translation: An<dyn Translation>,
    pub(crate) cache: An<dyn Candidate>,
}

impl Translation for CacheTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        todo!()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl CacheTranslation {
    fn new(translation: An<dyn Translation>) -> Self {
        todo!()
    }
}

#[inline]
fn cached<T, Args>(args: Args) -> An<dyn Translation> {
    An::new(CacheTranslation::new(T::from(args)))
}

struct DistinctTranslation {
    exhausted: bool,

    cache: CacheTranslation,
    pub(crate) candidate_set: BTreeSet<String>,
}

impl Translation for DistinctTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        todo!()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl DistinctTranslation {
    pub fn new(transition: An<dyn Translation>) -> Self {
        todo!()
    }
    pub(crate) fn already_has(text: &str) -> bool {
        todo!()
    }
}

struct PrefetchTranslation {
    exhausted: bool,

    pub(crate) transition: An<dyn Translation>,
    cache: CacheTranslation,
}

impl Translation for PrefetchTranslation {
    fn next(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> Option<An<dyn Candidate>> {
        todo!()
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn set_exhausted(&mut self, exhausted: bool) {
        self.exhausted = exhausted;
    }
}

impl PrefetchTranslation {
    pub fn new(transition: An<dyn Translation>) -> Self {
        todo!()
    }
    pub fn replenish(&self) -> bool {
        false
    }
}
