//! Writing systems — alphabet, syllabary, logographic, abjad, abugida.
//!
//! Pre-built metadata for major writing systems, queryable by ISO 15924 code.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl Script {
    /// Check if a Unicode code point falls within this script's ranges.
    #[must_use]
    pub fn contains_codepoint(&self, cp: u32) -> bool {
        self.unicode_ranges
            .iter()
            .any(|&(lo, hi)| cp >= lo && cp <= hi)
    }
}

/// Look up a script by ISO 15924 code.
///
/// Returns `None` for unregistered scripts.
#[must_use]
pub fn by_code(code: &str) -> Option<Script> {
    tracing::trace!(code, "script lookup");
    match code {
        "Latn" => Some(latin()),
        "Arab" => Some(arabic()),
        "Deva" => Some(devanagari()),
        "Hani" => Some(cjk()),
        "Cyrl" => Some(cyrillic()),
        "Hang" => Some(hangul()),
        "Kana" => Some(kana()),
        "Grek" => Some(greek()),
        _ => None,
    }
}

/// All registered script codes.
#[must_use]
pub fn all_codes() -> &'static [&'static str] {
    &[
        "Latn", "Arab", "Deva", "Hani", "Cyrl", "Hang", "Kana", "Grek",
    ]
}

// ---------------------------------------------------------------------------
// Pre-built script metadata
// ---------------------------------------------------------------------------

/// Latin script (ISO 15924: Latn).
#[must_use]
pub fn latin() -> Script {
    Script {
        code: Cow::Borrowed("Latn"),
        name: Cow::Borrowed("Latin"),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0x0041, 0x005A), // A-Z
            (0x0061, 0x007A), // a-z
            (0x00C0, 0x00FF), // Latin-1 Supplement (accented)
            (0x0100, 0x024F), // Latin Extended-A & B
        ],
        languages: vec![
            Cow::Borrowed("en"),
            Cow::Borrowed("fr"),
            Cow::Borrowed("es"),
            Cow::Borrowed("de"),
            Cow::Borrowed("pt"),
            Cow::Borrowed("it"),
        ],
    }
}

/// Arabic script (ISO 15924: Arab).
#[must_use]
pub fn arabic() -> Script {
    Script {
        code: Cow::Borrowed("Arab"),
        name: Cow::Borrowed("Arabic"),
        script_type: ScriptType::Abjad,
        direction: Direction::RightToLeft,
        unicode_ranges: vec![
            (0x0600, 0x06FF), // Arabic
            (0x0750, 0x077F), // Arabic Supplement
            (0xFB50, 0xFDFF), // Arabic Presentation Forms-A
            (0xFE70, 0xFEFF), // Arabic Presentation Forms-B
        ],
        languages: vec![
            Cow::Borrowed("ar"),
            Cow::Borrowed("fa"),
            Cow::Borrowed("ur"),
        ],
    }
}

/// Devanagari script (ISO 15924: Deva).
#[must_use]
pub fn devanagari() -> Script {
    Script {
        code: Cow::Borrowed("Deva"),
        name: Cow::Borrowed("Devanagari"),
        script_type: ScriptType::Abugida,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0x0900, 0x097F), // Devanagari
            (0xA8E0, 0xA8FF), // Devanagari Extended
        ],
        languages: vec![
            Cow::Borrowed("hi"),
            Cow::Borrowed("sa"),
            Cow::Borrowed("mr"),
            Cow::Borrowed("ne"),
        ],
    }
}

/// CJK Unified Ideographs (ISO 15924: Hani).
#[must_use]
pub fn cjk() -> Script {
    Script {
        code: Cow::Borrowed("Hani"),
        name: Cow::Borrowed("CJK Unified Ideographs"),
        script_type: ScriptType::Logographic,
        direction: Direction::LeftToRight, // modern default; historically TTB
        unicode_ranges: vec![
            (0x4E00, 0x9FFF),   // CJK Unified Ideographs
            (0x3400, 0x4DBF),   // CJK Unified Extension A
            (0x20000, 0x2A6DF), // CJK Unified Extension B
        ],
        languages: vec![
            Cow::Borrowed("zh"),
            Cow::Borrowed("ja"),
            Cow::Borrowed("ko"),
        ],
    }
}

/// Cyrillic script (ISO 15924: Cyrl).
#[must_use]
pub fn cyrillic() -> Script {
    Script {
        code: Cow::Borrowed("Cyrl"),
        name: Cow::Borrowed("Cyrillic"),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0x0400, 0x04FF), // Cyrillic
            (0x0500, 0x052F), // Cyrillic Supplement
        ],
        languages: vec![
            Cow::Borrowed("ru"),
            Cow::Borrowed("uk"),
            Cow::Borrowed("bg"),
            Cow::Borrowed("sr"),
        ],
    }
}

/// Hangul script (ISO 15924: Hang).
#[must_use]
pub fn hangul() -> Script {
    Script {
        code: Cow::Borrowed("Hang"),
        name: Cow::Borrowed("Hangul"),
        script_type: ScriptType::Alphabet, // featural alphabet
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0xAC00, 0xD7AF), // Hangul Syllables
            (0x1100, 0x11FF), // Hangul Jamo
            (0x3130, 0x318F), // Hangul Compatibility Jamo
        ],
        languages: vec![Cow::Borrowed("ko")],
    }
}

/// Japanese kana — Hiragana + Katakana (ISO 15924: Kana).
#[must_use]
pub fn kana() -> Script {
    Script {
        code: Cow::Borrowed("Kana"),
        name: Cow::Borrowed("Kana (Hiragana + Katakana)"),
        script_type: ScriptType::Syllabary,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0x3040, 0x309F), // Hiragana
            (0x30A0, 0x30FF), // Katakana
            (0x31F0, 0x31FF), // Katakana Phonetic Extensions
        ],
        languages: vec![Cow::Borrowed("ja")],
    }
}

/// Greek script (ISO 15924: Grek).
///
/// Used by sankhya for Greek mathematical notation.
#[must_use]
pub fn greek() -> Script {
    Script {
        code: Cow::Borrowed("Grek"),
        name: Cow::Borrowed("Greek"),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        unicode_ranges: vec![
            (0x0370, 0x03FF), // Greek and Coptic
            (0x1F00, 0x1FFF), // Greek Extended
        ],
        languages: vec![Cow::Borrowed("el")],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin_script() {
        let s = latin();
        assert_eq!(s.code, "Latn");
        assert_eq!(s.script_type, ScriptType::Alphabet);
        assert_eq!(s.direction, Direction::LeftToRight);
        assert!(s.contains_codepoint(0x0041)); // 'A'
        assert!(!s.contains_codepoint(0x0600)); // Arabic range
    }

    #[test]
    fn test_arabic_script() {
        let s = arabic();
        assert_eq!(s.code, "Arab");
        assert_eq!(s.script_type, ScriptType::Abjad);
        assert_eq!(s.direction, Direction::RightToLeft);
        assert!(s.contains_codepoint(0x0627)); // Arabic Alef
    }

    #[test]
    fn test_devanagari_script() {
        let s = devanagari();
        assert_eq!(s.code, "Deva");
        assert_eq!(s.script_type, ScriptType::Abugida);
        assert!(s.languages.iter().any(|l| l == "sa")); // Sanskrit uses Devanagari
    }

    #[test]
    fn test_cjk_script() {
        let s = cjk();
        assert_eq!(s.code, "Hani");
        assert_eq!(s.script_type, ScriptType::Logographic);
        assert!(s.contains_codepoint(0x4E00)); // first CJK unified
    }

    #[test]
    fn test_cyrillic_script() {
        let s = cyrillic();
        assert_eq!(s.code, "Cyrl");
        assert_eq!(s.script_type, ScriptType::Alphabet);
        assert!(s.contains_codepoint(0x0410)); // Cyrillic A
    }

    #[test]
    fn test_hangul_script() {
        let s = hangul();
        assert_eq!(s.code, "Hang");
        assert!(s.contains_codepoint(0xAC00)); // first Hangul syllable
    }

    #[test]
    fn test_kana_script() {
        let s = kana();
        assert_eq!(s.code, "Kana");
        assert_eq!(s.script_type, ScriptType::Syllabary);
        assert!(s.contains_codepoint(0x3042)); // Hiragana 'a'
        assert!(s.contains_codepoint(0x30A2)); // Katakana 'a'
    }

    #[test]
    fn test_greek_script() {
        let s = greek();
        assert_eq!(s.code, "Grek");
        assert_eq!(s.script_type, ScriptType::Alphabet);
        assert!(s.contains_codepoint(0x03B1)); // alpha
        assert!(s.contains_codepoint(0x03C0)); // pi
    }

    #[test]
    fn test_by_code_lookup() {
        assert!(by_code("Latn").is_some());
        assert!(by_code("Grek").is_some());
        assert!(by_code("XXXX").is_none());
    }

    #[test]
    fn test_all_codes_match_by_code() {
        for code in all_codes() {
            assert!(by_code(code).is_some(), "by_code failed for {code}");
        }
    }

    #[test]
    fn test_contains_codepoint_boundary() {
        let s = latin();
        // Exact boundaries
        assert!(s.contains_codepoint(0x0041)); // start of A-Z
        assert!(s.contains_codepoint(0x005A)); // end of A-Z
        assert!(!s.contains_codepoint(0x0040)); // just before
        assert!(!s.contains_codepoint(0x005B)); // just after
    }
}
