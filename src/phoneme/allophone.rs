//! Allophone rules — context-dependent sound variation.
//!
//! A phoneme may be realized as different surface sounds (allophones)
//! depending on its phonological environment. For example, English /t/
//! becomes \[ɾ\] (flap) between vowels ("water") and \[tʰ\] (aspirated)
//! word-initially ("top").

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// The phonological environment that triggers an allophone.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Environment {
    /// At the start of a word.
    WordInitial,
    /// At the end of a word.
    WordFinal,
    /// Between two vowels.
    Intervocalic,
    /// Before a vowel.
    PreVocalic,
    /// After a vowel.
    PostVocalic,
    /// Before a specific phoneme class.
    Before(PhonemeClass),
    /// After a specific phoneme class.
    After(PhonemeClass),
    /// At the end of a syllable (coda position).
    SyllableFinal,
    /// At the start of a syllable (onset position).
    SyllableInitial,
    /// Before a stressed syllable.
    PreStress,
    /// In an unstressed syllable.
    Unstressed,
}

/// A broad class of phonemes for environment matching.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PhonemeClass {
    /// Any vowel.
    Vowel,
    /// Any consonant.
    Consonant,
    /// Any nasal consonant.
    Nasal,
    /// Any plosive/stop consonant.
    Plosive,
    /// Any fricative consonant.
    Fricative,
    /// Any voiceless consonant.
    Voiceless,
    /// Any voiced sound.
    Voiced,
    /// A specific IPA symbol.
    Specific(Cow<'static, str>),
}

/// A rule mapping a phoneme to its allophonic realization in a given environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllophoneRule {
    /// The underlying phoneme (IPA).
    pub phoneme: Cow<'static, str>,
    /// The surface realization (IPA).
    pub allophone: Cow<'static, str>,
    /// The environment that triggers this realization.
    pub environment: Environment,
    /// Whether this rule is obligatory or optional.
    pub obligatory: bool,
}

/// A collection of allophone rules for a language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllophoneRuleSet {
    /// ISO 639 language code.
    pub language_code: Cow<'static, str>,
    /// Ordered rules (first matching rule wins).
    pub rules: Vec<AllophoneRule>,
}

impl AllophoneRuleSet {
    /// Find all allophone rules for a given phoneme.
    #[must_use]
    pub fn rules_for(&self, phoneme: &str) -> Vec<&AllophoneRule> {
        tracing::trace!(language = %self.language_code, phoneme, "allophone rule lookup");
        self.rules.iter().filter(|r| r.phoneme == phoneme).collect()
    }

    /// Find the allophone for a phoneme in a given environment.
    #[must_use]
    pub fn realize(&self, phoneme: &str, env: &Environment) -> Cow<'static, str> {
        tracing::trace!(
            language = %self.language_code,
            phoneme,
            environment = ?env,
            "allophone realization"
        );
        self.rules
            .iter()
            .find(|r| r.phoneme == phoneme && &r.environment == env)
            .map(|r| r.allophone.clone())
            .unwrap_or_else(|| Cow::Owned(phoneme.to_owned()))
    }
}

// ---------------------------------------------------------------------------
// Pre-built rule sets
// ---------------------------------------------------------------------------

/// English allophone rules (General American).
///
/// Note: aspiration rules use `WordInitial` as an approximation.
/// In precise phonology, aspiration occurs syllable-initially before
/// a stressed vowel (e.g., /p/ in "sport" is NOT aspirated).
#[must_use]
pub fn english_allophones() -> AllophoneRuleSet {
    AllophoneRuleSet {
        language_code: Cow::Borrowed("en"),
        rules: vec![
            // /t/ → [ɾ] intervocalic flapping ("water", "better")
            AllophoneRule {
                phoneme: Cow::Borrowed("t"),
                allophone: Cow::Borrowed("ɾ"),
                environment: Environment::Intervocalic,
                obligatory: false,
            },
            // /t/ → [tʰ] word-initial aspiration ("top")
            AllophoneRule {
                phoneme: Cow::Borrowed("t"),
                allophone: Cow::Borrowed("tʰ"),
                environment: Environment::WordInitial,
                obligatory: true,
            },
            // /p/ → [pʰ] word-initial aspiration ("pat")
            AllophoneRule {
                phoneme: Cow::Borrowed("p"),
                allophone: Cow::Borrowed("pʰ"),
                environment: Environment::WordInitial,
                obligatory: true,
            },
            // /k/ → [kʰ] word-initial aspiration ("cat")
            AllophoneRule {
                phoneme: Cow::Borrowed("k"),
                allophone: Cow::Borrowed("kʰ"),
                environment: Environment::WordInitial,
                obligatory: true,
            },
            // /l/ → [ɫ] dark-l in syllable-final ("feel", "milk")
            AllophoneRule {
                phoneme: Cow::Borrowed("l"),
                allophone: Cow::Borrowed("ɫ"),
                environment: Environment::SyllableFinal,
                obligatory: true,
            },
            // /ə/ → [ɨ] reduced vowel in unstressed syllables
            AllophoneRule {
                phoneme: Cow::Borrowed("ə"),
                allophone: Cow::Borrowed("ɨ"),
                environment: Environment::Unstressed,
                obligatory: false,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_allophones_count() {
        let rules = english_allophones();
        assert_eq!(rules.language_code, "en");
        assert!(!rules.rules.is_empty());
    }

    #[test]
    fn test_rules_for_t() {
        let rules = english_allophones();
        let t_rules = rules.rules_for("t");
        assert_eq!(t_rules.len(), 2); // intervocalic flap + aspiration
    }

    #[test]
    fn test_realize_t_intervocalic() {
        let rules = english_allophones();
        let result = rules.realize("t", &Environment::Intervocalic);
        assert_eq!(result, "ɾ");
    }

    #[test]
    fn test_realize_t_word_initial() {
        let rules = english_allophones();
        let result = rules.realize("t", &Environment::WordInitial);
        assert_eq!(result, "tʰ");
    }

    #[test]
    fn test_realize_unknown_env_returns_phoneme() {
        let rules = english_allophones();
        // /t/ has no rule for WordFinal, should return unchanged
        let result = rules.realize("t", &Environment::WordFinal);
        assert_eq!(result, "t");
    }

    #[test]
    fn test_realize_dark_l() {
        let rules = english_allophones();
        let result = rules.realize("l", &Environment::SyllableFinal);
        assert_eq!(result, "ɫ");
    }

    #[test]
    fn test_rules_for_unknown_phoneme() {
        let rules = english_allophones();
        assert!(rules.rules_for("ʀ").is_empty());
    }

    #[test]
    fn test_allophone_serde_roundtrip() {
        let rules = english_allophones();
        let json = serde_json::to_string(&rules).unwrap();
        let back: AllophoneRuleSet = serde_json::from_str(&json).unwrap();
        assert_eq!(rules, back);
    }
}
