//! Lipi — Multilingual Language Engine
//!
//! **Lipi** (Sanskrit: लिपि — script, writing system) provides a structured,
//! queryable corpus of human language data. Phoneme inventories, grapheme-to-phoneme
//! rules, writing system metadata, grammar structures, and lexicon access for
//! 50+ languages.
//!
//! # Architecture
//!
//! Four domain modules:
//!
//! - [`phoneme`] — IPA phoneme inventories per language, phonological features
//!   (manner, place, voicing), allophone rules, stress/tone patterns
//! - [`script`] — Writing system metadata: alphabet, syllabary, logographic,
//!   abjad, abugida. Character ranges, directionality, romanization tables
//! - [`grammar`] — Morphological typology (isolating, agglutinative, fusional),
//!   word order (SVO/SOV/VSO), case systems, verb conjugation patterns
//! - [`lexicon`] — Core vocabulary per language (Swadesh lists, frequency-ranked
//!   word lists), cognate detection, loanword tracking
//!
//! # Relationship to Other Crates
//!
//! ```text
//! lipi (this) — language structure & phoneme inventories
//!   ↓ provides phoneme sets per language
//! shabda — G2P conversion (currently English-only, lipi makes it multilingual)
//!   ↓ produces phoneme sequences
//! shabdakosh — pronunciation dictionary (currently CMUdict, lipi adds IPA dicts)
//!   ↓ lookup fallback
//! svara — vocal synthesis (consumes phonemes, produces audio)
//!   ↓ voice output
//! dhvani — audio engine (mixing, DSP, output)
//! ```
//!
//! Also feeds:
//! - **jnana** — multilingual knowledge access
//! - **vidya** — programming concepts explained in native languages
//! - **vansh** (planned) — voice assistant with multilingual TTS/STT
//! - **sahifa** (planned) — OCR language detection, multilingual document processing

pub mod error;
pub mod grammar;
pub mod lexicon;
pub mod phoneme;
pub mod registry;
pub mod script;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::LipiError;
