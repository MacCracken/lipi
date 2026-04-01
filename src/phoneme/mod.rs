//! Phoneme inventories — IPA phonemes per language, phonological features.
//!
//! Every language has a finite set of contrastive sounds (phonemes). This module
//! provides those inventories with IPA transcription, articulatory features
//! (manner, place, voicing), and language-specific allophone rules.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Articulatory manner of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Manner {
    Plosive,
    Nasal,
    Trill,
    TapFlap,
    Fricative,
    LateralFricative,
    Approximant,
    LateralApproximant,
    Affricate,
}

/// Place of articulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
    /// Simultaneous bilabial and velar (e.g., English /w/).
    LabialVelar,
}

/// Vowel height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

/// Vowel backness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Backness {
    Front,
    Central,
    Back,
}

/// A phoneme with its IPA symbol and articulatory features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Phoneme {
    /// IPA symbol (e.g., "p", "ʃ", "æ").
    pub ipa: Cow<'static, str>,
    /// Classification.
    pub kind: PhonemeKind,
}

impl Phoneme {
    /// Create a consonant phoneme.
    #[must_use]
    pub fn consonant(
        ipa: impl Into<Cow<'static, str>>,
        manner: Manner,
        place: Place,
        voiced: bool,
    ) -> Self {
        Self {
            ipa: ipa.into(),
            kind: PhonemeKind::Consonant {
                manner,
                place,
                voiced,
            },
        }
    }

    /// Create a vowel phoneme.
    #[must_use]
    pub fn vowel(
        ipa: impl Into<Cow<'static, str>>,
        height: Height,
        backness: Backness,
        rounded: bool,
    ) -> Self {
        Self {
            ipa: ipa.into(),
            kind: PhonemeKind::Vowel {
                height,
                backness,
                rounded,
            },
        }
    }
}

/// Classification of a phoneme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PhonemeKind {
    #[non_exhaustive]
    Consonant {
        manner: Manner,
        place: Place,
        voiced: bool,
    },
    #[non_exhaustive]
    Vowel {
        height: Height,
        backness: Backness,
        rounded: bool,
    },
}

/// A language's complete phoneme inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhonemeInventory {
    /// ISO 639-1 or 639-3 language code.
    pub language_code: Cow<'static, str>,
    /// Language name in English.
    pub language_name: Cow<'static, str>,
    /// All phonemes in this language.
    pub phonemes: Vec<Phoneme>,
    /// Tone system (None for non-tonal languages).
    pub tones: Option<Vec<Cow<'static, str>>>,
    /// Stress pattern.
    pub stress: StressPattern,
}

/// How stress works in a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StressPattern {
    /// Stress on a fixed syllable (e.g., French → final, Finnish → initial).
    Fixed,
    /// Stress is contrastive / unpredictable (e.g., English, Russian).
    Free,
    /// Pitch accent system (e.g., Japanese, Swedish).
    PitchAccent,
    /// Tonal — pitch distinguishes meaning (e.g., Mandarin, Yoruba).
    Tonal,
}

impl PhonemeInventory {
    /// Number of consonants in the inventory.
    #[must_use]
    #[inline]
    pub fn consonant_count(&self) -> usize {
        self.phonemes
            .iter()
            .filter(|p| matches!(p.kind, PhonemeKind::Consonant { .. }))
            .count()
    }

    /// Number of vowels in the inventory.
    #[must_use]
    #[inline]
    pub fn vowel_count(&self) -> usize {
        self.phonemes
            .iter()
            .filter(|p| matches!(p.kind, PhonemeKind::Vowel { .. }))
            .count()
    }

    /// Look up a phoneme by IPA symbol.
    #[must_use]
    pub fn find(&self, ipa: &str) -> Option<&Phoneme> {
        tracing::trace!(language = %self.language_code, ipa, "phoneme lookup");
        self.phonemes.iter().find(|p| p.ipa == ipa)
    }

    /// Check if a phoneme exists in this language.
    #[must_use]
    #[inline]
    pub fn has(&self, ipa: &str) -> bool {
        self.find(ipa).is_some()
    }
}

/// Build the English (General American) phoneme inventory.
#[must_use]
pub fn english() -> PhonemeInventory {
    tracing::debug!("building English (GA) phoneme inventory");
    PhonemeInventory {
        language_code: Cow::Borrowed("en"),
        language_name: Cow::Borrowed("English"),
        phonemes: vec![
            // Plosives
            Phoneme {
                ipa: Cow::Borrowed("p"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Bilabial,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("b"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Bilabial,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("t"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Alveolar,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("d"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Alveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("k"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Velar,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ɡ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Plosive,
                    place: Place::Velar,
                    voiced: true,
                },
            },
            // Fricatives
            Phoneme {
                ipa: Cow::Borrowed("f"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Labiodental,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("v"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Labiodental,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("θ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Dental,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ð"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Dental,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("s"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Alveolar,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("z"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Alveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ʃ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Postalveolar,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ʒ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Postalveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("h"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Fricative,
                    place: Place::Glottal,
                    voiced: false,
                },
            },
            // Nasals
            Phoneme {
                ipa: Cow::Borrowed("m"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Nasal,
                    place: Place::Bilabial,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("n"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Nasal,
                    place: Place::Alveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ŋ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Nasal,
                    place: Place::Velar,
                    voiced: true,
                },
            },
            // Approximants
            Phoneme {
                ipa: Cow::Borrowed("ɹ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Approximant,
                    place: Place::Alveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("l"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::LateralApproximant,
                    place: Place::Alveolar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("w"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Approximant,
                    place: Place::LabialVelar,
                    voiced: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("j"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Approximant,
                    place: Place::Palatal,
                    voiced: true,
                },
            },
            // Affricates
            Phoneme {
                ipa: Cow::Borrowed("t͡ʃ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Affricate,
                    place: Place::Postalveolar,
                    voiced: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("d͡ʒ"),
                kind: PhonemeKind::Consonant {
                    manner: Manner::Affricate,
                    place: Place::Postalveolar,
                    voiced: true,
                },
            },
            // Vowels (General American)
            Phoneme {
                ipa: Cow::Borrowed("iː"),
                kind: PhonemeKind::Vowel {
                    height: Height::Close,
                    backness: Backness::Front,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ɪ"),
                kind: PhonemeKind::Vowel {
                    height: Height::NearClose,
                    backness: Backness::Front,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("eɪ"),
                kind: PhonemeKind::Vowel {
                    height: Height::CloseMid,
                    backness: Backness::Front,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ɛ"),
                kind: PhonemeKind::Vowel {
                    height: Height::OpenMid,
                    backness: Backness::Front,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("æ"),
                kind: PhonemeKind::Vowel {
                    height: Height::NearOpen,
                    backness: Backness::Front,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ɑː"),
                kind: PhonemeKind::Vowel {
                    height: Height::Open,
                    backness: Backness::Back,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ɔː"),
                kind: PhonemeKind::Vowel {
                    height: Height::OpenMid,
                    backness: Backness::Back,
                    rounded: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("oʊ"),
                kind: PhonemeKind::Vowel {
                    height: Height::CloseMid,
                    backness: Backness::Back,
                    rounded: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ʊ"),
                kind: PhonemeKind::Vowel {
                    height: Height::NearClose,
                    backness: Backness::Back,
                    rounded: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("uː"),
                kind: PhonemeKind::Vowel {
                    height: Height::Close,
                    backness: Backness::Back,
                    rounded: true,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ʌ"),
                kind: PhonemeKind::Vowel {
                    height: Height::OpenMid,
                    backness: Backness::Central,
                    rounded: false,
                },
            },
            Phoneme {
                ipa: Cow::Borrowed("ə"),
                kind: PhonemeKind::Vowel {
                    height: Height::Mid,
                    backness: Backness::Central,
                    rounded: false,
                },
            },
        ],
        tones: None,
        stress: StressPattern::Free,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_inventory_size() {
        let en = english();
        // English: ~24 consonants, ~11-15 vowels (GA)
        assert!(en.consonant_count() >= 22);
        assert!(en.vowel_count() >= 10);
    }

    #[test]
    fn test_english_has_th() {
        let en = english();
        assert!(en.has("θ")); // voiceless dental fricative (think)
        assert!(en.has("ð")); // voiced dental fricative (this)
    }

    #[test]
    fn test_english_no_tones() {
        let en = english();
        assert!(en.tones.is_none());
        assert_eq!(en.stress, StressPattern::Free);
    }

    #[test]
    fn test_find_phoneme() {
        let en = english();
        let p = en.find("ʃ").unwrap();
        assert!(matches!(
            p.kind,
            PhonemeKind::Consonant {
                manner: Manner::Fricative,
                place: Place::Postalveolar,
                voiced: false
            }
        ));
    }

    #[test]
    fn test_missing_phoneme() {
        let en = english();
        // English doesn't have the uvular trill
        assert!(!en.has("ʀ"));
    }

    #[test]
    fn test_w_is_labial_velar() {
        let en = english();
        let w = en.find("w").unwrap();
        assert!(matches!(
            w.kind,
            PhonemeKind::Consonant {
                place: Place::LabialVelar,
                ..
            }
        ));
    }

    #[test]
    fn test_phoneme_eq() {
        let a = Phoneme {
            ipa: Cow::Borrowed("p"),
            kind: PhonemeKind::Consonant {
                manner: Manner::Plosive,
                place: Place::Bilabial,
                voiced: false,
            },
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
