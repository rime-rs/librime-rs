use crate::algo::spelling::Spelling;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::vec::Vec;

use super::spelling::SpellingProperties;
struct Calculation;
struct Schema;

pub struct Script {
    map: HashMap<String, Vec<Spelling>>,
}

impl Script {
    // 添加音节
    pub fn add_syllable(&mut self, syllable: String) -> bool {
        if self.map.contains_key(&syllable) {
            return false;
        }

        let spelling = Spelling::new(syllable.clone());

        self.map
            .entry(syllable)
            .or_insert_with(Vec::new)
            .push(spelling);

        true
    }

    // 合并音节
    // pub fn merge(&mut self, s: String, sp: SpellingProperties, v: Vec<spelling::Spelling>) {
    //     let m = self.map.entry(s).or_insert_with(Vec::new);
    //     for x in v {
    //         let mut y = x.clone();
    //         {
    //             let yy = &mut y.properties;
    //             if sp.spelling_type > yy.spelling_type {
    //                 yy.spelling_type = sp.spelling_type;
    //             }
    //             yy.credibility += sp.credibility;
    //             if !sp.tips.is_empty() {
    //                 yy.tips = sp.tips.clone();
    //             }
    //         }
    //         if let Some(e) = m.iter_mut().find(|e| e == &&y) {
    //             let zz = &mut e.properties;
    //             if y.properties.spelling_type < zz.spelling_type {
    //                 zz.spelling_type = y.properties.spelling_type;
    //             }
    //             if y.properties.credibility > zz.credibility {
    //                 zz.credibility = y.properties.credibility;
    //             }
    //             zz.tips.clear();
    //         } else {
    //             m.push(y);
    //         }
    //     }
    // }

    // 导出数据到文件
    // pub fn dump(&self, file_path: &Path) {
    //     let mut file = File::create(file_path)?;
    //     for (key, spellings) in &self.map {
    //         let mut first = true;

    //         // 遍历每个 Spelling
    //         for s in spellings {
    //             // 写入键（仅在第一行）
    //             if first {
    //                 write!(file, "{}", key)?;
    //                 first = false;
    //             } else {
    //                 write!(file, "")?; // 空字符串占位
    //             }

    //             // 写入 Spelling 的内容、类型、可信度和提示
    //             write!(
    //                 file,
    //                 "\t{}\t{}\t{}\t{}\n",
    //                 s.str,
    //                 Self::type_to_char(s.properties.spelling_type),
    //                 s.properties.credibility,
    //                 s.properties.tips.join(", ") // 将 tips 拼接为字符串
    //             )?;
    //         }
    //     }
    // }

    // 将 type_ 转换为字符
    fn type_to_char(type_: u32) -> char {
        match type_ {
            0 => '-',
            1 => 'a',
            2 => 'c',
            3 => '?',
            4 => '!',
            _ => ' ', // 默认值
        }
    }
}
