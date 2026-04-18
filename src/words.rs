use ahash::AHashMap;
use brotli::Decompressor;
use once_cell::sync::Lazy;
use std::io::{Cursor, Read};

use crate::WordList;

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

generate_lazy_db_from_file!(standard);
generate_lazy_db_from_file!(nerd);
generate_lazy_db_from_file!(challenge);

#[inline(always)]
pub(crate) fn get(lang: WordList) -> &'static Words {
    match lang {
        WordList::Nerd => &NERD,
        WordList::Standard => &STANDARD,
        WordList::Challenge => &CHALLENGE,
    }
}

#[inline(always)]
pub(crate) fn get_len(len: usize, lang: WordList) -> Option<&'static Words> {
    match lang {
        WordList::Nerd => NERD_LEN.get(&len),
        WordList::Standard => STANDARD_LEN.get(&len),
        WordList::Challenge => CHALLENGE_LEN.get(&len),
    }
}

#[inline(always)]
pub(crate) fn get_starts_with(char: char, lang: WordList) -> Option<&'static Words> {
    match lang {
        WordList::Standard => STANDARD_STARTS_WITH.get(&char),
        WordList::Challenge => CHALLENGE_STARTS_WITH.get(&char),
        WordList::Nerd => NERD_STARTS_WITH.get(&char),
    }
}
