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

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Builder for constructing [`PhonemeInventory`] instances.
///
/// # Example
/// ```
/// use lipi::phoneme::*;
///
/// let inv = PhonemeInventoryBuilder::new("xx", "Example")
///     .stress(StressPattern::Fixed)
///     .consonant("p", Manner::Plosive, Place::Bilabial, false)
///     .vowel("a", Height::Open, Backness::Central, false)
///     .build();
///
/// assert_eq!(inv.consonant_count(), 1);
/// assert_eq!(inv.vowel_count(), 1);
/// ```
pub struct PhonemeInventoryBuilder {
    language_code: Cow<'static, str>,
    language_name: Cow<'static, str>,
    phonemes: Vec<Phoneme>,
    tones: Option<Vec<Cow<'static, str>>>,
    stress: StressPattern,
}

impl PhonemeInventoryBuilder {
    /// Start building an inventory for the given language.
    #[must_use]
    pub fn new(code: impl Into<Cow<'static, str>>, name: impl Into<Cow<'static, str>>) -> Self {
        Self::with_capacity(code, name, 48)
    }

    /// Start building with a specific phoneme capacity hint.
    #[must_use]
    pub fn with_capacity(
        code: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        capacity: usize,
    ) -> Self {
        Self {
            language_code: code.into(),
            language_name: name.into(),
            phonemes: Vec::with_capacity(capacity),
            tones: None,
            stress: StressPattern::Free,
        }
    }

    /// Set the stress pattern.
    #[must_use]
    pub fn stress(mut self, pattern: StressPattern) -> Self {
        self.stress = pattern;
        self
    }

    /// Set the tone system.
    #[must_use]
    pub fn tones(mut self, tones: Vec<Cow<'static, str>>) -> Self {
        self.tones = Some(tones);
        self
    }

    /// Add a consonant.
    #[must_use]
    pub fn consonant(
        mut self,
        ipa: impl Into<Cow<'static, str>>,
        manner: Manner,
        place: Place,
        voiced: bool,
    ) -> Self {
        self.phonemes
            .push(Phoneme::consonant(ipa, manner, place, voiced));
        self
    }

    /// Add a vowel.
    #[must_use]
    pub fn vowel(
        mut self,
        ipa: impl Into<Cow<'static, str>>,
        height: Height,
        backness: Backness,
        rounded: bool,
    ) -> Self {
        self.phonemes
            .push(Phoneme::vowel(ipa, height, backness, rounded));
        self
    }

    /// Add a pre-built phoneme.
    #[must_use]
    pub fn phoneme(mut self, phoneme: Phoneme) -> Self {
        self.phonemes.push(phoneme);
        self
    }

    /// Consume the builder and produce the inventory.
    #[must_use]
    pub fn build(self) -> PhonemeInventory {
        tracing::debug!(
            language = %self.language_code,
            phonemes = self.phonemes.len(),
            "built phoneme inventory"
        );
        PhonemeInventory {
            language_code: self.language_code,
            language_name: self.language_name,
            phonemes: self.phonemes,
            tones: self.tones,
            stress: self.stress,
        }
    }
}

// ---------------------------------------------------------------------------
// Pre-built inventories
// ---------------------------------------------------------------------------

/// Build the English (General American) phoneme inventory.
///
/// 24 consonants + 12 vowels. Stress: free (contrastive).
#[must_use]
pub fn english() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("en", "English", 36)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("ʃ", Fricative, Postalveolar, false)
        .consonant("ʒ", Fricative, Postalveolar, true)
        .consonant("h", Fricative, Glottal, false)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        .consonant("ŋ", Nasal, Velar, true)
        // Approximants
        .consonant("ɹ", Approximant, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("w", Approximant, LabialVelar, true)
        .consonant("j", Approximant, Palatal, true)
        // Affricates
        .consonant("t͡ʃ", Affricate, Postalveolar, false)
        .consonant("d͡ʒ", Affricate, Postalveolar, true)
        // Vowels (General American)
        .vowel("iː", Close, Front, false)
        .vowel("ɪ", NearClose, Front, false)
        .vowel("eɪ", CloseMid, Front, false)
        .vowel("ɛ", OpenMid, Front, false)
        .vowel("æ", NearOpen, Front, false)
        .vowel("ɑː", Open, Back, false)
        .vowel("ɔː", OpenMid, Back, true)
        .vowel("oʊ", CloseMid, Back, true)
        .vowel("ʊ", NearClose, Back, true)
        .vowel("uː", Close, Back, true)
        .vowel("ʌ", OpenMid, Central, false)
        .vowel("ə", Mid, Central, false)
        .build()
}

/// Build the Sanskrit (Classical) phoneme inventory.
///
/// Sanskrit has a systematic phoneme inventory organized by place and manner,
/// critical for the Katapayadi numeral encoding system used by sankhya.
///
/// 36 consonants + 14 vowels. Stress: pitch accent.
#[must_use]
pub fn sanskrit() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("sa", "Sanskrit", 51)
        .stress(StressPattern::PitchAccent)
        // --- Sparsha (stop/plosive) consonants: 5 vargas × 5 ---
        // Kavarga (velar)
        .consonant("k", Plosive, Velar, false)
        .consonant("kʰ", Plosive, Velar, false) // aspirated
        .consonant("ɡ", Plosive, Velar, true)
        .consonant("ɡʰ", Plosive, Velar, true) // aspirated
        .consonant("ŋ", Nasal, Velar, true)
        // Chavarga (palatal)
        .consonant("t͡ɕ", Affricate, Palatal, false)
        .consonant("t͡ɕʰ", Affricate, Palatal, false)
        .consonant("d͡ʑ", Affricate, Palatal, true)
        .consonant("d͡ʑʰ", Affricate, Palatal, true)
        .consonant("ɲ", Nasal, Palatal, true)
        // Tavarga (retroflex)
        .consonant("ʈ", Plosive, Retroflex, false)
        .consonant("ʈʰ", Plosive, Retroflex, false)
        .consonant("ɖ", Plosive, Retroflex, true)
        .consonant("ɖʰ", Plosive, Retroflex, true)
        .consonant("ɳ", Nasal, Retroflex, true)
        // Tavarga (dental)
        .consonant("t̪", Plosive, Dental, false)
        .consonant("t̪ʰ", Plosive, Dental, false)
        .consonant("d̪", Plosive, Dental, true)
        .consonant("d̪ʰ", Plosive, Dental, true)
        .consonant("n̪", Nasal, Dental, true)
        // Pavarga (bilabial)
        .consonant("p", Plosive, Bilabial, false)
        .consonant("pʰ", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("bʰ", Plosive, Bilabial, true)
        .consonant("m", Nasal, Bilabial, true)
        // --- Antastha (semivowels/approximants) ---
        .consonant("j", Approximant, Palatal, true)
        .consonant("r", Trill, Alveolar, true)
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("ʋ", Approximant, Labiodental, true)
        // --- Ushman (sibilants/fricatives) ---
        .consonant("ɕ", Fricative, Palatal, false)
        .consonant("ʂ", Fricative, Retroflex, false)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("ɦ", Fricative, Glottal, true)
        // --- Additional ---
        .consonant("ɭ", LateralApproximant, Retroflex, true) // Vedic ḷ
        .consonant("kʂ", Affricate, Velar, false) // kṣa
        .consonant("d͡ʑɲ", Affricate, Palatal, true) // jña
        // --- Vowels (svara) ---
        // Short
        .vowel("ɐ", NearOpen, Central, false) // a
        .vowel("i", Close, Front, false) // i
        .vowel("u", Close, Back, true) // u
        .vowel("r̩", Close, Central, false) // ṛ (syllabic r)
        .vowel("l̩", Close, Central, false) // ḷ (syllabic l)
        // Long
        .vowel("ɐː", NearOpen, Central, false) // ā
        .vowel("iː", Close, Front, false) // ī
        .vowel("uː", Close, Back, true) // ū
        .vowel("r̩ː", Close, Central, false) // ṝ
        .vowel("l̩ː", Close, Central, false) // ḹ
        // Diphthongs
        .vowel("eː", CloseMid, Front, false) // e
        .vowel("ɐi", NearOpen, Front, false) // ai
        .vowel("oː", CloseMid, Back, true) // o
        .vowel("ɐu", NearOpen, Back, true) // au
        // Anusvāra and visarga are suprasegmental, not separate phonemes
        .build()
}

/// Build the Greek (Modern Standard) phoneme inventory.
///
/// Provides the phoneme set needed for Greek mathematical notation
/// and script metadata used by sankhya.
///
/// 20 consonants + 5 vowels. Stress: free (contrastive).
#[must_use]
pub fn greek() -> PhonemeInventory {
    use Backness::*;
    use Height::*;
    use Manner::*;
    use Place::*;

    PhonemeInventoryBuilder::with_capacity("el", "Greek", 25)
        .stress(StressPattern::Free)
        // Plosives
        .consonant("p", Plosive, Bilabial, false)
        .consonant("b", Plosive, Bilabial, true)
        .consonant("t", Plosive, Alveolar, false)
        .consonant("d", Plosive, Alveolar, true)
        .consonant("k", Plosive, Velar, false)
        .consonant("ɡ", Plosive, Velar, true)
        // Fricatives
        .consonant("f", Fricative, Labiodental, false)
        .consonant("v", Fricative, Labiodental, true)
        .consonant("θ", Fricative, Dental, false)
        .consonant("ð", Fricative, Dental, true)
        .consonant("s", Fricative, Alveolar, false)
        .consonant("z", Fricative, Alveolar, true)
        .consonant("x", Fricative, Velar, false)
        .consonant("ɣ", Fricative, Velar, true)
        // Nasals
        .consonant("m", Nasal, Bilabial, true)
        .consonant("n", Nasal, Alveolar, true)
        // Liquids
        .consonant("l", LateralApproximant, Alveolar, true)
        .consonant("r", Trill, Alveolar, true)
        // Affricates
        .consonant("t͡s", Affricate, Alveolar, false)
        .consonant("d͡z", Affricate, Alveolar, true)
        // Vowels (5-vowel system)
        .vowel("i", Close, Front, false)
        .vowel("e", CloseMid, Front, false)
        .vowel("a", Open, Central, false)
        .vowel("o", CloseMid, Back, true)
        .vowel("u", Close, Back, true)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- English tests --

    #[test]
    fn test_english_inventory_size() {
        let en = english();
        assert_eq!(en.consonant_count(), 24);
        assert_eq!(en.vowel_count(), 12);
    }

    #[test]
    fn test_english_has_th() {
        let en = english();
        assert!(en.has("θ"));
        assert!(en.has("ð"));
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
        let a = Phoneme::consonant("p", Manner::Plosive, Place::Bilabial, false);
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- Sanskrit tests --

    #[test]
    fn test_sanskrit_inventory_size() {
        let sa = sanskrit();
        assert_eq!(sa.consonant_count(), 36);
        assert_eq!(sa.vowel_count(), 14);
        assert_eq!(sa.language_code, "sa");
    }

    #[test]
    fn test_sanskrit_five_vargas() {
        let sa = sanskrit();
        // Each varga has 5 consonants: unvoiced, unvoiced-aspirated,
        // voiced, voiced-aspirated, nasal
        // Kavarga (velar)
        assert!(sa.has("k"));
        assert!(sa.has("kʰ"));
        assert!(sa.has("ɡ"));
        assert!(sa.has("ɡʰ"));
        assert!(sa.has("ŋ"));
    }

    #[test]
    fn test_sanskrit_retroflexes() {
        let sa = sanskrit();
        assert!(sa.has("ʈ"));
        assert!(sa.has("ɖ"));
        assert!(sa.has("ɳ"));
    }

    #[test]
    fn test_sanskrit_sibilants() {
        let sa = sanskrit();
        assert!(sa.has("ɕ")); // palatal
        assert!(sa.has("ʂ")); // retroflex
        assert!(sa.has("s")); // alveolar
    }

    #[test]
    fn test_sanskrit_vowels() {
        let sa = sanskrit();
        // Syllabic r and l are distinctive Sanskrit vowels
        assert!(sa.has("r̩"));
        assert!(sa.has("l̩"));
        // Short/long pairs
        assert!(sa.has("ɐ"));
        assert!(sa.has("ɐː"));
    }

    #[test]
    fn test_sanskrit_stress() {
        let sa = sanskrit();
        assert_eq!(sa.stress, StressPattern::PitchAccent);
        assert!(sa.tones.is_none());
    }

    // -- Greek tests --

    #[test]
    fn test_greek_inventory_size() {
        let el = greek();
        assert_eq!(el.consonant_count(), 20);
        assert_eq!(el.vowel_count(), 5);
        assert_eq!(el.language_code, "el");
    }

    #[test]
    fn test_greek_five_vowels() {
        let el = greek();
        assert!(el.has("i"));
        assert!(el.has("e"));
        assert!(el.has("a"));
        assert!(el.has("o"));
        assert!(el.has("u"));
    }

    #[test]
    fn test_greek_velar_fricatives() {
        let el = greek();
        assert!(el.has("x"));
        assert!(el.has("ɣ"));
    }

    #[test]
    fn test_greek_stress() {
        let el = greek();
        assert_eq!(el.stress, StressPattern::Free);
    }

    // -- Builder tests --

    #[test]
    fn test_builder_minimal() {
        let inv = PhonemeInventoryBuilder::new("xx", "Test")
            .consonant("t", Manner::Plosive, Place::Alveolar, false)
            .vowel("a", Height::Open, Backness::Central, false)
            .build();
        assert_eq!(inv.language_code, "xx");
        assert_eq!(inv.consonant_count(), 1);
        assert_eq!(inv.vowel_count(), 1);
        assert_eq!(inv.stress, StressPattern::Free); // default
    }

    #[test]
    fn test_builder_with_tones() {
        let inv = PhonemeInventoryBuilder::new("xx", "Tonal Test")
            .stress(StressPattern::Tonal)
            .tones(vec![
                Cow::Borrowed("˥"),
                Cow::Borrowed("˧˥"),
                Cow::Borrowed("˨˩˦"),
                Cow::Borrowed("˥˩"),
            ])
            .vowel("a", Height::Open, Backness::Central, false)
            .build();
        assert_eq!(inv.stress, StressPattern::Tonal);
        assert_eq!(inv.tones.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_builder_phoneme_method() {
        let custom = Phoneme::consonant("ɬ", Manner::LateralFricative, Place::Alveolar, false);
        let inv = PhonemeInventoryBuilder::new("xx", "Test")
            .phoneme(custom.clone())
            .build();
        assert_eq!(inv.phonemes[0], custom);
    }
}
