//! Writing systems — alphabet, syllabary, logographic, abjad, abugida.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Writing system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ScriptType {
    /// Each symbol = one phoneme (e.g., Latin, Cyrillic, Greek).
    Alphabet,
    /// Each symbol = consonant + inherent vowel, modified by diacritics (e.g., Devanagari, Thai).
    Abugida,
    /// Each symbol = consonant only, vowels optional/diacritical (e.g., Arabic, Hebrew).
    Abjad,
    /// Each symbol = one syllable (e.g., Japanese kana, Cherokee).
    Syllabary,
    /// Each symbol = one morpheme/word (e.g., Chinese hanzi, Japanese kanji).
    Logographic,
    /// Mixed system (e.g., Japanese uses all three).
    Mixed,
}

/// Text directionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    /// Bidirectional (e.g., mixed Arabic + Latin text).
    Bidirectional,
}

/// A writing system's metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    /// ISO 15924 code (e.g., "Latn", "Arab", "Deva").
    pub code: Cow<'static, str>,
    /// Human name (e.g., "Latin", "Arabic", "Devanagari").
    pub name: Cow<'static, str>,
    /// Classification.
    pub script_type: ScriptType,
    /// Primary text direction.
    pub direction: Direction,
    /// Unicode block ranges (start, end) for this script.
    pub unicode_ranges: Vec<(u32, u32)>,
    /// Languages that use this script.
    pub languages: Vec<Cow<'static, str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_types() {
        let latin = Script {
            code: Cow::Borrowed("Latn"),
            name: Cow::Borrowed("Latin"),
            script_type: ScriptType::Alphabet,
            direction: Direction::LeftToRight,
            unicode_ranges: vec![(0x0041, 0x005A), (0x0061, 0x007A)],
            languages: vec![
                Cow::Borrowed("en"),
                Cow::Borrowed("fr"),
                Cow::Borrowed("es"),
                Cow::Borrowed("de"),
            ],
        };
        assert_eq!(latin.script_type, ScriptType::Alphabet);
        assert_eq!(latin.direction, Direction::LeftToRight);
    }

    #[test]
    fn test_arabic_rtl() {
        let arabic = Script {
            code: Cow::Borrowed("Arab"),
            name: Cow::Borrowed("Arabic"),
            script_type: ScriptType::Abjad,
            direction: Direction::RightToLeft,
            unicode_ranges: vec![(0x0600, 0x06FF)],
            languages: vec![
                Cow::Borrowed("ar"),
                Cow::Borrowed("fa"),
                Cow::Borrowed("ur"),
            ],
        };
        assert_eq!(arabic.direction, Direction::RightToLeft);
        assert_eq!(arabic.script_type, ScriptType::Abjad);
    }
}
