//! Transliteration and romanization tables.
//!
//! Bidirectional mapping between native script characters and their
//! romanized representations. Supports multiple schemes per script
//! (e.g., Devanagari → IAST vs ISO 15919).

use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A named transliteration scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransliterationTable {
    /// Scheme name (e.g., "IAST", "ISO 15919", "Beta Code").
    pub scheme: Cow<'static, str>,
    /// Source script code (ISO 15924).
    pub source_script: Cow<'static, str>,
    /// Target script code (ISO 15924), typically "Latn" for romanization.
    pub target_script: Cow<'static, str>,
    /// Forward mappings: source character → target string.
    pub forward: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl TransliterationTable {
    /// Transliterate a single character/grapheme from source to target.
    #[must_use]
    pub fn transliterate_char(&self, ch: &str) -> Option<&str> {
        self.forward
            .iter()
            .find(|(src, _)| src.as_ref() == ch)
            .map(|(_, tgt)| tgt.as_ref())
    }

    /// Build a reverse mapping (target → source) for this table.
    ///
    /// When multiple source characters map to the same target (e.g.,
    /// Devanagari standalone vowel and matra form), the last mapping wins.
    #[must_use]
    pub fn reverse_map(&self) -> HashMap<&str, &str> {
        self.forward
            .iter()
            .map(|(src, tgt)| (tgt.as_ref(), src.as_ref()))
            .collect()
    }

    /// Transliterate a string by greedily matching longest source sequences.
    #[must_use]
    pub fn transliterate(&self, input: &str) -> String {
        tracing::trace!(scheme = %self.scheme, "transliterating");
        let mut result = String::with_capacity(input.len());
        let mut remaining = input;

        while !remaining.is_empty() {
            // Byte offsets after each of the first 4 chars
            let char_ends: Vec<usize> = remaining
                .char_indices()
                .take(4)
                .map(|(i, ch)| i + ch.len_utf8())
                .collect();

            let mut matched = false;
            // Try longest match first
            for &end in char_ends.iter().rev() {
                let candidate = &remaining[..end];
                if let Some(replacement) = self.transliterate_char(candidate) {
                    result.push_str(replacement);
                    remaining = &remaining[end..];
                    matched = true;
                    break;
                }
            }
            if !matched {
                // No mapping found — pass through first char unchanged
                result.push_str(&remaining[..char_ends[0]]);
                remaining = &remaining[char_ends[0]..];
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// Pre-built tables
// ---------------------------------------------------------------------------

/// Devanagari to IAST (International Alphabet of Sanskrit Transliteration).
#[must_use]
pub fn devanagari_iast() -> TransliterationTable {
    TransliterationTable {
        scheme: Cow::Borrowed("IAST"),
        source_script: Cow::Borrowed("Deva"),
        target_script: Cow::Borrowed("Latn"),
        forward: vec![
            // Vowels
            (Cow::Borrowed("अ"), Cow::Borrowed("a")),
            (Cow::Borrowed("आ"), Cow::Borrowed("ā")),
            (Cow::Borrowed("इ"), Cow::Borrowed("i")),
            (Cow::Borrowed("ई"), Cow::Borrowed("ī")),
            (Cow::Borrowed("उ"), Cow::Borrowed("u")),
            (Cow::Borrowed("ऊ"), Cow::Borrowed("ū")),
            (Cow::Borrowed("ऋ"), Cow::Borrowed("ṛ")),
            (Cow::Borrowed("ॠ"), Cow::Borrowed("ṝ")),
            (Cow::Borrowed("ऌ"), Cow::Borrowed("ḷ")),
            (Cow::Borrowed("ॡ"), Cow::Borrowed("ḹ")),
            (Cow::Borrowed("ए"), Cow::Borrowed("e")),
            (Cow::Borrowed("ऐ"), Cow::Borrowed("ai")),
            (Cow::Borrowed("ओ"), Cow::Borrowed("o")),
            (Cow::Borrowed("औ"), Cow::Borrowed("au")),
            // Vowel signs (matras)
            (Cow::Borrowed("ा"), Cow::Borrowed("ā")),
            (Cow::Borrowed("ि"), Cow::Borrowed("i")),
            (Cow::Borrowed("ी"), Cow::Borrowed("ī")),
            (Cow::Borrowed("ु"), Cow::Borrowed("u")),
            (Cow::Borrowed("ू"), Cow::Borrowed("ū")),
            (Cow::Borrowed("ृ"), Cow::Borrowed("ṛ")),
            (Cow::Borrowed("े"), Cow::Borrowed("e")),
            (Cow::Borrowed("ै"), Cow::Borrowed("ai")),
            (Cow::Borrowed("ो"), Cow::Borrowed("o")),
            (Cow::Borrowed("ौ"), Cow::Borrowed("au")),
            // Kavarga (velar)
            (Cow::Borrowed("क"), Cow::Borrowed("ka")),
            (Cow::Borrowed("ख"), Cow::Borrowed("kha")),
            (Cow::Borrowed("ग"), Cow::Borrowed("ga")),
            (Cow::Borrowed("घ"), Cow::Borrowed("gha")),
            (Cow::Borrowed("ङ"), Cow::Borrowed("ṅa")),
            // Chavarga (palatal)
            (Cow::Borrowed("च"), Cow::Borrowed("ca")),
            (Cow::Borrowed("छ"), Cow::Borrowed("cha")),
            (Cow::Borrowed("ज"), Cow::Borrowed("ja")),
            (Cow::Borrowed("झ"), Cow::Borrowed("jha")),
            (Cow::Borrowed("ञ"), Cow::Borrowed("ña")),
            // Tavarga (retroflex)
            (Cow::Borrowed("ट"), Cow::Borrowed("ṭa")),
            (Cow::Borrowed("ठ"), Cow::Borrowed("ṭha")),
            (Cow::Borrowed("ड"), Cow::Borrowed("ḍa")),
            (Cow::Borrowed("ढ"), Cow::Borrowed("ḍha")),
            (Cow::Borrowed("ण"), Cow::Borrowed("ṇa")),
            // Tavarga (dental)
            (Cow::Borrowed("त"), Cow::Borrowed("ta")),
            (Cow::Borrowed("थ"), Cow::Borrowed("tha")),
            (Cow::Borrowed("द"), Cow::Borrowed("da")),
            (Cow::Borrowed("ध"), Cow::Borrowed("dha")),
            (Cow::Borrowed("न"), Cow::Borrowed("na")),
            // Pavarga (bilabial)
            (Cow::Borrowed("प"), Cow::Borrowed("pa")),
            (Cow::Borrowed("फ"), Cow::Borrowed("pha")),
            (Cow::Borrowed("ब"), Cow::Borrowed("ba")),
            (Cow::Borrowed("भ"), Cow::Borrowed("bha")),
            (Cow::Borrowed("म"), Cow::Borrowed("ma")),
            // Semivowels
            (Cow::Borrowed("य"), Cow::Borrowed("ya")),
            (Cow::Borrowed("र"), Cow::Borrowed("ra")),
            (Cow::Borrowed("ल"), Cow::Borrowed("la")),
            (Cow::Borrowed("व"), Cow::Borrowed("va")),
            // Sibilants/fricatives
            (Cow::Borrowed("श"), Cow::Borrowed("śa")),
            (Cow::Borrowed("ष"), Cow::Borrowed("ṣa")),
            (Cow::Borrowed("स"), Cow::Borrowed("sa")),
            (Cow::Borrowed("ह"), Cow::Borrowed("ha")),
            // Special
            (Cow::Borrowed("ं"), Cow::Borrowed("ṃ")), // anusvara
            (Cow::Borrowed("ः"), Cow::Borrowed("ḥ")), // visarga
            (Cow::Borrowed("्"), Cow::Borrowed("")),  // virama (halant)
            (Cow::Borrowed("ॐ"), Cow::Borrowed("oṃ")), // om
        ],
    }
}

/// Greek to Beta Code transliteration.
///
/// Beta Code is a widely-used ASCII encoding for Greek text in classics
/// and digital humanities.
#[must_use]
pub fn greek_beta_code() -> TransliterationTable {
    TransliterationTable {
        scheme: Cow::Borrowed("Beta Code"),
        source_script: Cow::Borrowed("Grek"),
        target_script: Cow::Borrowed("Latn"),
        forward: vec![
            // Lowercase
            (Cow::Borrowed("α"), Cow::Borrowed("a")),
            (Cow::Borrowed("β"), Cow::Borrowed("b")),
            (Cow::Borrowed("γ"), Cow::Borrowed("g")),
            (Cow::Borrowed("δ"), Cow::Borrowed("d")),
            (Cow::Borrowed("ε"), Cow::Borrowed("e")),
            (Cow::Borrowed("ζ"), Cow::Borrowed("z")),
            (Cow::Borrowed("η"), Cow::Borrowed("h")),
            (Cow::Borrowed("θ"), Cow::Borrowed("q")),
            (Cow::Borrowed("ι"), Cow::Borrowed("i")),
            (Cow::Borrowed("κ"), Cow::Borrowed("k")),
            (Cow::Borrowed("λ"), Cow::Borrowed("l")),
            (Cow::Borrowed("μ"), Cow::Borrowed("m")),
            (Cow::Borrowed("ν"), Cow::Borrowed("n")),
            (Cow::Borrowed("ξ"), Cow::Borrowed("c")),
            (Cow::Borrowed("ο"), Cow::Borrowed("o")),
            (Cow::Borrowed("π"), Cow::Borrowed("p")),
            (Cow::Borrowed("ρ"), Cow::Borrowed("r")),
            (Cow::Borrowed("σ"), Cow::Borrowed("s")),
            (Cow::Borrowed("ς"), Cow::Borrowed("s")), // final sigma
            (Cow::Borrowed("τ"), Cow::Borrowed("t")),
            (Cow::Borrowed("υ"), Cow::Borrowed("u")),
            (Cow::Borrowed("φ"), Cow::Borrowed("f")),
            (Cow::Borrowed("χ"), Cow::Borrowed("x")),
            (Cow::Borrowed("ψ"), Cow::Borrowed("y")),
            (Cow::Borrowed("ω"), Cow::Borrowed("w")),
            // Uppercase
            (Cow::Borrowed("Α"), Cow::Borrowed("*a")),
            (Cow::Borrowed("Β"), Cow::Borrowed("*b")),
            (Cow::Borrowed("Γ"), Cow::Borrowed("*g")),
            (Cow::Borrowed("Δ"), Cow::Borrowed("*d")),
            (Cow::Borrowed("Ε"), Cow::Borrowed("*e")),
            (Cow::Borrowed("Ζ"), Cow::Borrowed("*z")),
            (Cow::Borrowed("Η"), Cow::Borrowed("*h")),
            (Cow::Borrowed("Θ"), Cow::Borrowed("*q")),
            (Cow::Borrowed("Ι"), Cow::Borrowed("*i")),
            (Cow::Borrowed("Κ"), Cow::Borrowed("*k")),
            (Cow::Borrowed("Λ"), Cow::Borrowed("*l")),
            (Cow::Borrowed("Μ"), Cow::Borrowed("*m")),
            (Cow::Borrowed("Ν"), Cow::Borrowed("*n")),
            (Cow::Borrowed("Ξ"), Cow::Borrowed("*c")),
            (Cow::Borrowed("Ο"), Cow::Borrowed("*o")),
            (Cow::Borrowed("Π"), Cow::Borrowed("*p")),
            (Cow::Borrowed("Ρ"), Cow::Borrowed("*r")),
            (Cow::Borrowed("Σ"), Cow::Borrowed("*s")),
            (Cow::Borrowed("Τ"), Cow::Borrowed("*t")),
            (Cow::Borrowed("Υ"), Cow::Borrowed("*u")),
            (Cow::Borrowed("Φ"), Cow::Borrowed("*f")),
            (Cow::Borrowed("Χ"), Cow::Borrowed("*x")),
            (Cow::Borrowed("Ψ"), Cow::Borrowed("*y")),
            (Cow::Borrowed("Ω"), Cow::Borrowed("*w")),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devanagari_iast_char() {
        let table = devanagari_iast();
        assert_eq!(table.transliterate_char("अ"), Some("a"));
        assert_eq!(table.transliterate_char("आ"), Some("ā"));
        assert_eq!(table.transliterate_char("क"), Some("ka"));
    }

    #[test]
    fn test_devanagari_iast_unknown_char() {
        let table = devanagari_iast();
        assert_eq!(table.transliterate_char("X"), None);
    }

    #[test]
    fn test_devanagari_iast_transliterate() {
        let table = devanagari_iast();
        // "नमस्ते" → "namaste" (simplified: न म स् ते)
        assert_eq!(table.transliterate("अ"), "a");
        assert_eq!(table.transliterate("क"), "ka");
    }

    #[test]
    fn test_devanagari_iast_reverse_map() {
        let table = devanagari_iast();
        let rev = table.reverse_map();
        assert_eq!(rev.get("a"), Some(&"अ"));
        assert_eq!(rev.get("ā"), Some(&"ा")); // matra form (last match wins)
    }

    #[test]
    fn test_greek_beta_code_char() {
        let table = greek_beta_code();
        assert_eq!(table.transliterate_char("α"), Some("a"));
        assert_eq!(table.transliterate_char("π"), Some("p"));
        assert_eq!(table.transliterate_char("Σ"), Some("*s"));
    }

    #[test]
    fn test_greek_beta_code_transliterate() {
        let table = greek_beta_code();
        assert_eq!(table.transliterate("λογος"), "logos");
        assert_eq!(table.transliterate("Αθηνα"), "*aqhna");
    }

    #[test]
    fn test_passthrough_unmapped() {
        let table = greek_beta_code();
        // Numbers and spaces should pass through unchanged
        assert_eq!(table.transliterate("α 1"), "a 1");
    }

    #[test]
    fn test_transliteration_serde_roundtrip() {
        let table = devanagari_iast();
        let json = serde_json::to_string(&table).unwrap();
        let back: TransliterationTable = serde_json::from_str(&json).unwrap();
        assert_eq!(table, back);
    }
}
