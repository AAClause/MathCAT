//! Private-use characters for internal braille post-processing markers.
//!
//! These must not overlap with Mathematical Alphanumeric Symbols that can appear in MathML.
//! Previously MathCAT reused code points from that block (e.g. U+1D44F `𝑏`), which caused
//! panics when real math content contained the same characters.
//!
//! The constants below document the assigned code points. The same characters also appear as
//! literals in phf maps, regexes, and Braille Rules YAML after conversion.
#![allow(dead_code)]

/// Lone decimal point hack (was mathematical italic capital N, U+1D441).
pub const LONE_DECIMAL: char = '\u{F8E0}';
/// Punctuation after a roman numeral (was mathematical bold capital P, U+1D40F).
pub const ROMAN_PUNCT: char = '\u{F8E1}';
/// Highlighted baseline indicator (was mathematical italic small b, U+1D44F).
pub const BASELINE_HIGHLIGHT: char = '\u{F8E2}';
/// Capital letter that should not get a word indicator (was mathematical bold capital C, U+1D436).
pub const CAP_NO_WORD: char = '\u{F8E3}';
/// Hard whitespace break between expressions (was mathematical bold capital W, U+1D416).
pub const WHITESPACE_HARD: char = '\u{F8E4}';
/// CMU whitespace insertion marker (was mathematical sans-serif bold italic w, U+1D604).
pub const CMU_ADD_SPACE: char = '\u{F8E5}';
/// Second/later braille cell of a capital letter (was mathematical italic small c, U+1D450).
pub const CAP_CONTINUE: char = '\u{F8E6}';
/// Soft whitespace around relational operators (was mathematical bold small w, U+1D430).
pub const WHITESPACE_SOFT: char = '\u{F8E7}';
