use ahash::AHashMap;
use brotli::Decompressor;
use once_cell::sync::Lazy;
use std::io::{Cursor, Read};

use crate::Lang;

pub(crate) type Words = Box<[&'static str]>;

macro_rules! generate_lazy_db_from_file {
    ($file_stem:ident) => {
        paste::paste! {
            static [<$file_stem:upper _COMPRESSED>]: Lazy<String> = Lazy::new(|| {
                let compressed_bytes = include_bytes!(concat!("br/", stringify!($file_stem), ".br"));
                let cursor = Cursor::new(compressed_bytes);
                let mut decompressor = Decompressor::new(cursor, 4096);

                let mut decompressed_bytes = Vec::new();
                decompressor.read_to_end(&mut decompressed_bytes).expect("Decompression failed");

                let decompressed_string = String::from_utf8(decompressed_bytes)
                    .expect("Decompression resulted in invalid UTF-8");

                decompressed_string
            });

            static [<$file_stem:upper>]: Lazy<Words> = Lazy::new(|| {
                [<$file_stem:upper _COMPRESSED>].lines().collect()
            });

            static [<$file_stem:upper _LEN>]: Lazy<AHashMap<usize, Words>> = Lazy::new(|| {
                let mut map = AHashMap::new();

                for &word in [<$file_stem:upper>].iter() {
                    let len = word.chars().count();
                    map.entry(len).or_insert_with(Vec::new).push(word);
                }

                map.into_iter().map(|(k, v)| (k, v.into_boxed_slice())).collect()
            });

            static [<$file_stem:upper _STARTS_WITH>]: Lazy<AHashMap<char, Words>> = Lazy::new(|| {
                let mut map = AHashMap::new();

                for &word in [<$file_stem:upper>].iter() {
                    let first = word.chars().nth(0).expect("empty word");
                    map.entry(first).or_insert_with(Vec::new).push(word);
                }

                map.into_iter().map(|(k, v)| (k, v.into_boxed_slice())).collect()
            });
        }
    };
}

generate_lazy_db_from_file!(sw);
generate_lazy_db_from_file!(fw);
generate_lazy_db_from_file!(nw);

#[inline(always)]
pub(crate) fn get(lang: Lang) -> &'static Words {
    match lang {
        Lang::Simple => &SW,
        Lang::Full => &FW,
        Lang::Nerd => &NW,
    }
}

#[inline(always)]
pub(crate) fn get_len(len: usize, lang: Lang) -> Option<&'static Words> {
    match lang {
        Lang::Simple => SW_LEN.get(&len),
        Lang::Full => FW_LEN.get(&len),
        Lang::Nerd => NW_LEN.get(&len),
    }
}

#[inline(always)]
pub(crate) fn get_starts_with(char: char, lang: Lang) -> Option<&'static Words> {
    match lang {
        Lang::Simple => SW_STARTS_WITH.get(&char),
        Lang::Full => FW_STARTS_WITH.get(&char),
        Lang::Nerd => NW_STARTS_WITH.get(&char),
    }
}
