//! Syllable structure and phonotactic constraints.
//!
//! Languages differ in what syllable shapes they permit. Japanese allows
//! only (C)V(N), while English permits complex onsets and codas like
//! /stɹ/ and /ŋkθs/. This module models those constraints.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A syllable structure template describing the maximum complexity
/// a language permits in onset, nucleus, and coda positions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyllableTemplate {
    /// Maximum consonants in the onset (before the vowel).
    pub max_onset: u8,
    /// Maximum consonants in the coda (after the vowel).
    pub max_coda: u8,
    /// Whether complex nuclei (diphthongs/long vowels) are permitted.
    pub complex_nucleus: bool,
    /// Canonical syllable pattern (e.g., "(C)(C)(C)V(C)(C)(C)(C)").
    pub pattern: Cow<'static, str>,
}

impl SyllableTemplate {
    /// Whether this language allows consonant clusters in the onset.
    #[must_use]
    #[inline]
    pub fn allows_onset_clusters(&self) -> bool {
        self.max_onset > 1
    }

    /// Whether this language allows consonant clusters in the coda.
    #[must_use]
    #[inline]
    pub fn allows_coda_clusters(&self) -> bool {
        self.max_coda > 1
    }

    /// Whether the language allows closed syllables (non-zero coda).
    #[must_use]
    #[inline]
    pub fn allows_closed_syllables(&self) -> bool {
        self.max_coda > 0
    }
}

/// A phonotactic constraint describing permitted or forbidden
/// sound sequences within a language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhonotacticConstraint {
    /// What kind of constraint this is.
    pub kind: ConstraintKind,
    /// The position in the syllable this constraint applies to.
    pub position: SyllablePosition,
    /// IPA sequences involved (e.g., ["st", "sp", "sk"] for valid English onsets).
    pub sequences: Vec<Cow<'static, str>>,
    /// Human-readable description.
    pub description: Cow<'static, str>,
}

/// Whether a constraint permits or forbids sequences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConstraintKind {
    /// These sequences are permitted.
    Permitted,
    /// These sequences are forbidden.
    Forbidden,
}

/// Position in the syllable where a constraint applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SyllablePosition {
    Onset,
    Coda,
    Nucleus,
    /// Across a syllable boundary.
    AcrossBoundary,
}

/// Complete phonotactic profile for a language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Phonotactics {
    /// ISO 639 language code.
    pub language_code: Cow<'static, str>,
    /// Syllable structure template.
    pub syllable: SyllableTemplate,
    /// Phonotactic constraints.
    pub constraints: Vec<PhonotacticConstraint>,
}

impl Phonotactics {
    /// Get all constraints for a given syllable position.
    #[must_use]
    pub fn constraints_at(&self, position: SyllablePosition) -> Vec<&PhonotacticConstraint> {
        tracing::trace!(
            language = %self.language_code,
            position = ?position,
            "phonotactic constraint lookup"
        );
        self.constraints
            .iter()
            .filter(|c| c.position == position)
            .collect()
    }

    /// Check if a sequence is explicitly permitted at a given position.
    ///
    /// Returns `Some(true)` if found in a `Permitted` constraint,
    /// `Some(false)` if found in a `Forbidden` constraint, or
    /// `None` if no constraint mentions this sequence. The first
    /// matching constraint wins (order-sensitive).
    #[must_use]
    pub fn is_permitted(&self, sequence: &str, position: SyllablePosition) -> Option<bool> {
        for c in &self.constraints {
            if c.position != position {
                continue;
            }
            let matches = c.sequences.iter().any(|s| s.as_ref() == sequence);
            if matches {
                return Some(c.kind == ConstraintKind::Permitted);
            }
        }
        None // no rule found
    }
}

// ---------------------------------------------------------------------------
// Pre-built phonotactics
// ---------------------------------------------------------------------------

/// English (General American) phonotactic profile.
#[must_use]
pub fn english_phonotactics() -> Phonotactics {
    Phonotactics {
        language_code: Cow::Borrowed("en"),
        syllable: SyllableTemplate {
            max_onset: 3,
            max_coda: 4,
            complex_nucleus: true,
            pattern: Cow::Borrowed("(C)(C)(C)V(C)(C)(C)(C)"),
        },
        constraints: vec![
            PhonotacticConstraint {
                kind: ConstraintKind::Permitted,
                position: SyllablePosition::Onset,
                sequences: vec![
                    Cow::Borrowed("st"),
                    Cow::Borrowed("sp"),
                    Cow::Borrowed("sk"),
                    Cow::Borrowed("str"),
                    Cow::Borrowed("spr"),
                    Cow::Borrowed("skr"),
                    Cow::Borrowed("spl"),
                    Cow::Borrowed("skw"),
                    Cow::Borrowed("stj"),
                    Cow::Borrowed("spj"),
                    Cow::Borrowed("skj"),
                ],
                description: Cow::Borrowed("English /s/+stop onset clusters"),
            },
            PhonotacticConstraint {
                kind: ConstraintKind::Forbidden,
                position: SyllablePosition::Onset,
                sequences: vec![
                    Cow::Borrowed("sr"),
                    Cow::Borrowed("tl"),
                    Cow::Borrowed("dl"),
                    Cow::Borrowed("ŋ"),
                ],
                description: Cow::Borrowed("Forbidden English onset sequences"),
            },
        ],
    }
}

/// Sanskrit phonotactic profile.
#[must_use]
pub fn sanskrit_phonotactics() -> Phonotactics {
    Phonotactics {
        language_code: Cow::Borrowed("sa"),
        syllable: SyllableTemplate {
            max_onset: 2,
            max_coda: 2,
            complex_nucleus: true,
            pattern: Cow::Borrowed("(C)(C)V(C)(C)"),
        },
        constraints: vec![PhonotacticConstraint {
            kind: ConstraintKind::Permitted,
            position: SyllablePosition::Onset,
            sequences: vec![
                Cow::Borrowed("pr"),
                Cow::Borrowed("kr"),
                Cow::Borrowed("tr"),
                Cow::Borrowed("sr"),
                Cow::Borrowed("pl"),
                Cow::Borrowed("kl"),
            ],
            description: Cow::Borrowed("Sanskrit stop+liquid onset clusters"),
        }],
    }
}

/// Japanese phonotactic profile.
#[must_use]
pub fn japanese_phonotactics() -> Phonotactics {
    Phonotactics {
        language_code: Cow::Borrowed("ja"),
        syllable: SyllableTemplate {
            max_onset: 1,
            max_coda: 1,
            complex_nucleus: true,
            pattern: Cow::Borrowed("(C)V(N)"),
        },
        constraints: vec![PhonotacticConstraint {
            kind: ConstraintKind::Permitted,
            position: SyllablePosition::Coda,
            sequences: vec![Cow::Borrowed("n"), Cow::Borrowed("ɴ")],
            description: Cow::Borrowed("Only moraic nasal permitted in Japanese coda"),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_syllable_template() {
        let p = english_phonotactics();
        assert_eq!(p.syllable.max_onset, 3);
        assert_eq!(p.syllable.max_coda, 4);
        assert!(p.syllable.allows_onset_clusters());
        assert!(p.syllable.allows_coda_clusters());
        assert!(p.syllable.allows_closed_syllables());
    }

    #[test]
    fn test_japanese_syllable_template() {
        let p = japanese_phonotactics();
        assert_eq!(p.syllable.max_onset, 1);
        assert_eq!(p.syllable.max_coda, 1);
        assert!(!p.syllable.allows_onset_clusters());
        assert!(!p.syllable.allows_coda_clusters());
        assert!(p.syllable.allows_closed_syllables());
    }

    #[test]
    fn test_english_onset_permitted() {
        let p = english_phonotactics();
        assert_eq!(p.is_permitted("str", SyllablePosition::Onset), Some(true));
        assert_eq!(p.is_permitted("sp", SyllablePosition::Onset), Some(true));
    }

    #[test]
    fn test_english_onset_forbidden() {
        let p = english_phonotactics();
        assert_eq!(p.is_permitted("sr", SyllablePosition::Onset), Some(false));
        assert_eq!(p.is_permitted("tl", SyllablePosition::Onset), Some(false));
    }

    #[test]
    fn test_english_onset_unknown() {
        let p = english_phonotactics();
        // "br" is valid in English but not in our explicit lists
        assert_eq!(p.is_permitted("br", SyllablePosition::Onset), None);
    }

    #[test]
    fn test_constraints_at() {
        let p = english_phonotactics();
        let onset = p.constraints_at(SyllablePosition::Onset);
        assert_eq!(onset.len(), 2); // permitted + forbidden
        let coda = p.constraints_at(SyllablePosition::Coda);
        assert!(coda.is_empty());
    }

    #[test]
    fn test_sanskrit_onset() {
        let p = sanskrit_phonotactics();
        assert_eq!(p.is_permitted("kr", SyllablePosition::Onset), Some(true));
        assert_eq!(p.syllable.max_onset, 2);
    }

    #[test]
    fn test_japanese_coda() {
        let p = japanese_phonotactics();
        assert_eq!(p.is_permitted("n", SyllablePosition::Coda), Some(true));
    }

    #[test]
    fn test_phonotactics_serde_roundtrip() {
        let p = english_phonotactics();
        let json = serde_json::to_string(&p).unwrap();
        let back: Phonotactics = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn test_sanskrit_phonotactics_serde_roundtrip() {
        let p = sanskrit_phonotactics();
        let json = serde_json::to_string(&p).unwrap();
        let back: Phonotactics = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }
}
