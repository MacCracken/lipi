//! Lexicon — core vocabulary, Swadesh lists, frequency data,
//! cognate tracking, and etymology.

pub mod cognate;
pub mod swadesh;

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A lexical entry — one word in one language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LexEntry {
    /// The word in its native script.
    pub word: Cow<'static, str>,
    /// IPA transcription.
    pub ipa: Cow<'static, str>,
    /// English gloss.
    pub gloss: Cow<'static, str>,
    /// Part of speech.
    pub pos: PartOfSpeech,
    /// Frequency rank (lower = more common). None if unknown.
    pub frequency_rank: Option<u32>,
    /// Swadesh list index (1-207). None if not a Swadesh word.
    pub swadesh_index: Option<u16>,
}

/// Part of speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Particle,
    Numeral,
}

/// A language's lexicon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lexicon {
    /// ISO 639 language code.
    pub language_code: Cow<'static, str>,
    /// All entries.
    pub entries: Vec<LexEntry>,
}

impl Lexicon {
    /// Look up a word by its native form.
    #[must_use]
    pub fn find(&self, word: &str) -> Option<&LexEntry> {
        tracing::trace!(language = %self.language_code, word, "lexicon lookup");
        self.entries.iter().find(|e| e.word == word)
    }

    /// Get all Swadesh list entries, sorted by index.
    #[must_use]
    pub fn swadesh(&self) -> Vec<&LexEntry> {
        tracing::trace!(language = %self.language_code, "extracting Swadesh list");
        let mut result: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.swadesh_index.is_some())
            .collect();
        result.sort_by_key(|e| e.swadesh_index.unwrap_or(u16::MAX));
        result
    }

    /// Get the N most frequent words.
    #[must_use]
    pub fn most_frequent(&self, n: usize) -> Vec<&LexEntry> {
        tracing::trace!(language = %self.language_code, n, "frequency ranking");
        let mut ranked: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.frequency_rank.is_some())
            .collect();
        ranked.sort_by_key(|e| e.frequency_rank.unwrap_or(u32::MAX));
        ranked.into_iter().take(n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lexicon() -> Lexicon {
        Lexicon {
            language_code: Cow::Borrowed("en"),
            entries: vec![
                LexEntry {
                    word: Cow::Borrowed("water"),
                    ipa: Cow::Borrowed("ˈwɔːtər"),
                    gloss: Cow::Borrowed("water"),
                    pos: PartOfSpeech::Noun,
                    frequency_rank: Some(250),
                    swadesh_index: Some(1),
                },
                LexEntry {
                    word: Cow::Borrowed("fire"),
                    ipa: Cow::Borrowed("ˈfaɪər"),
                    gloss: Cow::Borrowed("fire"),
                    pos: PartOfSpeech::Noun,
                    frequency_rank: Some(800),
                    swadesh_index: Some(2),
                },
                LexEntry {
                    word: Cow::Borrowed("the"),
                    ipa: Cow::Borrowed("ðə"),
                    gloss: Cow::Borrowed("the (definite article)"),
                    pos: PartOfSpeech::Determiner,
                    frequency_rank: Some(1),
                    swadesh_index: None,
                },
            ],
        }
    }

    #[test]
    fn test_find_word() {
        let lex = sample_lexicon();
        assert!(lex.find("water").is_some());
        assert!(lex.find("xyz").is_none());
    }

    #[test]
    fn test_swadesh_list() {
        let lex = sample_lexicon();
        let sw = lex.swadesh();
        assert_eq!(sw.len(), 2);
        assert_eq!(sw[0].word, "water");
    }

    #[test]
    fn test_most_frequent() {
        let lex = sample_lexicon();
        let freq = lex.most_frequent(2);
        assert_eq!(freq[0].word, "the");
        assert_eq!(freq[1].word, "water");
    }

    #[test]
    fn test_find_empty_lexicon() {
        let lex = Lexicon {
            language_code: Cow::Borrowed("xx"),
            entries: vec![],
        };
        assert!(lex.find("anything").is_none());
        assert!(lex.swadesh().is_empty());
        assert!(lex.most_frequent(10).is_empty());
    }

    #[test]
    fn test_lex_entry_eq() {
        let a = LexEntry {
            word: Cow::Borrowed("cat"),
            ipa: Cow::Borrowed("kæt"),
            gloss: Cow::Borrowed("cat"),
            pos: PartOfSpeech::Noun,
            frequency_rank: Some(500),
            swadesh_index: None,
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
