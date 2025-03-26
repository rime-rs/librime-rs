// #![allow(unused)]
pub mod algo;
pub mod candidate;
pub mod config;
pub mod deployer;
pub mod dict;
pub mod gear;
pub mod translation;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod common;
mod lever;
mod message;

#[cfg(test)]
mod tests;
