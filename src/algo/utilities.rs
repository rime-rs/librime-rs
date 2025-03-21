use crc32fast::Hasher;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub(crate) fn compare_version_string(x: &str, y: &str) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let x_parts: Vec<&str> = x.split('.').collect();
    let y_parts: Vec<&str> = y.split('.').collect();

    while i < x_parts.len() || j < y_parts.len() {
        let v1 = if i < x_parts.len() {
            x_parts[i].parse::<i32>().unwrap_or(0)
        } else {
            0
        };
        let v2 = if j < y_parts.len() {
            y_parts[j].parse::<i32>().unwrap_or(0)
        } else {
            0
        };
        if v1 > v2 {
            return 1;
        } else if v1 < v2 {
            return -1;
        }
        i += 1;
        j += 1;
    }
    0
}

pub(crate) struct ChecksumComputer {
    crc: Hasher,
}

impl ChecksumComputer {
    pub fn new(initial_remainder: u32) -> Self {
        let mut crc = Hasher::new();
        crc.update(&[initial_remainder as u8]);
        ChecksumComputer { crc }
    }

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
pub(crate) fn checksum(file_path: &Path) -> u32 {
    let mut c = ChecksumComputer::new(0);
    if let Err(e) = c.process_file(file_path) {
        eprintln!("Error processing file: {}", e);
        return 0;
    }
    c.checksum()
}
