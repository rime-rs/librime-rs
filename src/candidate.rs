use std::any::Any;

pub trait Candidates: Any {
    fn as_any(&self) -> &dyn Any; // 用于向下转换
    fn r#type(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn quality(&self) -> f64;
}

#[derive(Default, Debug)]
pub struct Candidate {
    r#type: String,
    start: usize,
    end: usize,
    quality: f64,
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
