pub mod algo;
pub mod candidate;
pub mod config;
pub mod dict;
pub mod gear;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod common;

#[cfg(test)]
mod tests;
