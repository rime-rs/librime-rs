use crc32fast::Hasher;
use regex::Regex;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn compare_version_string(x: &str, y: &str) -> i32 {
    let re = Regex::new(r"[.-]").unwrap();
    let x_parts: Vec<&str> = re.split(x).collect();
    let y_parts: Vec<&str> = re.split(y).collect();

    for i in 0..x_parts.len().max(y_parts.len()) {
        let x_num = x_parts.get(i).and_then(extract_leading_number).unwrap_or(0);
        let y_num = y_parts.get(i).and_then(extract_leading_number).unwrap_or(0);

        match x_num.cmp(&y_num) {
            Ordering::Greater => return 1,
            Ordering::Less => return -1,
            _ => {}
        }
    }
    0
}

fn extract_leading_number(s: &&str) -> Option<i32> {
    s.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}
pub struct ChecksumComputer {
    crc: Hasher,
}

impl From<u32> for ChecksumComputer {
    fn from(initial_remainder: u32) -> Self {
        let mut crc = Hasher::new();
        crc.update(&[initial_remainder as u8]);
        ChecksumComputer { crc }
    }
}

impl ChecksumComputer {
    pub fn process_file(&mut self, file_path: &Path) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.crc.update(&buffer);
        Ok(())
    }

    pub fn checksum(&self) -> u32 {
        self.crc.clone().finalize()
    }
}

#[inline]
pub fn checksum(file_path: &Path) -> u32 {
    let mut c = ChecksumComputer::from(0);
    c.process_file(file_path).expect("File processing failed");
    c.checksum()
}
