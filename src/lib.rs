//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate
//! random words. Included words can be filtered by length or
//! first character.
//!
//! ## Usage
//! You **MUST** enable a crate language feature.
//! Crate language features are mandatory to reduce binary size.
//! Example for English in Cargo.toml:
//! ```toml
//! [dependencies]
//! random_word = { version = "0.5.0", features = ["en"] }
//! ```
//!
//! **Supported Languages:**
//! - Wordle
//!

mod words;

use rand::{seq::SliceRandom, thread_rng};

/// ISO 639-1 language codes.
///
/// Each variant corresponds to a
/// set of words included in the binary.
///
/// You **MUST** enable the corresponding crate feature.
///
/// # Variants
///
/// * `Simple` - Wordle for those of us who like to win.
/// * `Full` - They say it's 'english' but I have questions.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordList {
    Standard,
    Challenge,
    Nerd,
}

impl From<&str> for WordList {
    fn from(thing: &str) -> Self {
        match thing.to_ascii_lowercase().as_str() {
            "challenge" => WordList::Challenge,
            "nerd" => WordList::Nerd,
            "standard" => WordList::Standard,
            _ => WordList::Standard,
        }
    }
}

/// Returns all words with the given language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let words = random_word::all(Lang::En);
/// assert!(!words.is_empty());
/// ```
#[inline(always)]
pub fn all(lang: WordList) -> &'static [&'static str] {
    words::get(lang)
}

/// Returns a random word with the given language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::get(Lang::En);
/// assert!(!word.is_empty());
/// ```
#[inline(always)]
pub fn get(lang: WordList) -> &'static str {
    words::get(lang)
        .choose(&mut thread_rng())
        .expect("array is empty")
}

/// Returns all words with the given length and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let words = random_word::all_len(4, Lang::En);
/// assert!(words.is_some());
/// ```
#[inline(always)]
pub fn all_len(len: usize, lang: WordList) -> Option<&'static [&'static str]> {
    words::get_len(len, lang).map(|boxed| &**boxed)
}

/// Returns a random word with the given length and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::get_len(4, Lang::En);
/// assert!(word.is_some());
/// ```
#[inline(always)]
pub fn get_len(len: usize, lang: WordList) -> Option<&'static str> {
    words::get_len(len, lang)?
        .choose(&mut thread_rng())
        .copied()
}

/// Returns all words with the given starting character and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let words = random_word::all_starts_with('c', Lang::En);
/// assert!(words.is_some());
/// ```
#[inline(always)]
pub fn all_starts_with(char: char, lang: WordList) -> Option<&'static [&'static str]> {
    words::get_starts_with(char, lang).map(|boxed| &**boxed)
}

/// Returns a random word with the given starting character and language.
///
/// # Example
/// ```
/// use random_word::Lang;
/// let word = random_word::get_starts_with('c', Lang::En);
/// assert!(word.is_some());
/// ```
#[inline(always)]
pub fn get_starts_with(char: char, lang: WordList) -> Option<&'static str> {
    words::get_starts_with(char, lang)?
        .choose(&mut thread_rng())
        .copied()
}
