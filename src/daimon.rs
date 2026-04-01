//! Daimon agent registration — capability advertisement for the AGNOS agent framework.
//!
//! Defines lipi's capabilities so that daimon can discover and orchestrate
//! language-related operations. This module produces registration data;
//! the actual daimon client integration lives in the consuming application.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A capability that lipi exposes to the agent framework.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capability {
    /// Capability name (e.g., "phoneme_lookup").
    pub name: Cow<'static, str>,
    /// Human-readable description.
    pub description: Cow<'static, str>,
    /// Input parameter names.
    pub inputs: Vec<Cow<'static, str>>,
    /// Output type description.
    pub output: Cow<'static, str>,
}

/// Lipi's agent registration data for daimon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRegistration {
    /// Agent name.
    pub name: Cow<'static, str>,
    /// Version string.
    pub version: Cow<'static, str>,
    /// Description of what this agent provides.
    pub description: Cow<'static, str>,
    /// Capabilities this agent exposes.
    pub capabilities: Vec<Capability>,
    /// Language codes this agent has data for.
    pub supported_languages: Vec<Cow<'static, str>>,
    /// Script codes this agent has data for.
    pub supported_scripts: Vec<Cow<'static, str>>,
}

/// Build lipi's agent registration for daimon.
#[must_use]
pub fn registration() -> AgentRegistration {
    let lang_codes: Vec<Cow<'static, str>> = crate::registry::all_codes()
        .iter()
        .map(|&c| Cow::Borrowed(c))
        .collect();
    let script_codes: Vec<Cow<'static, str>> = crate::script::all_codes()
        .iter()
        .map(|&c| Cow::Borrowed(c))
        .collect();

    AgentRegistration {
        name: Cow::Borrowed("lipi"),
        version: Cow::Borrowed(env!("CARGO_PKG_VERSION")),
        description: Cow::Borrowed(
            "Multilingual language engine: phoneme inventories, writing systems, \
             grammar profiles, and lexicon access",
        ),
        capabilities: vec![
            Capability {
                name: Cow::Borrowed("phoneme_lookup"),
                description: Cow::Borrowed(
                    "Look up phoneme inventory for a language by ISO 639 code",
                ),
                inputs: vec![Cow::Borrowed("language_code")],
                output: Cow::Borrowed("PhonemeInventory"),
            },
            Capability {
                name: Cow::Borrowed("script_lookup"),
                description: Cow::Borrowed("Look up writing system metadata by ISO 15924 code"),
                inputs: vec![Cow::Borrowed("script_code")],
                output: Cow::Borrowed("Script"),
            },
            Capability {
                name: Cow::Borrowed("grammar_profile"),
                description: Cow::Borrowed(
                    "Get grammatical profile (morphology, word order, cases) for a language",
                ),
                inputs: vec![Cow::Borrowed("language_code")],
                output: Cow::Borrowed("GrammarProfile"),
            },
            Capability {
                name: Cow::Borrowed("transliterate"),
                description: Cow::Borrowed(
                    "Transliterate text between scripts (Devanagari↔IAST, Greek↔Beta Code)",
                ),
                inputs: vec![Cow::Borrowed("text"), Cow::Borrowed("scheme")],
                output: Cow::Borrowed("String"),
            },
            Capability {
                name: Cow::Borrowed("language_compare"),
                description: Cow::Borrowed(
                    "Compare phoneme inventories and grammar between two languages",
                ),
                inputs: vec![Cow::Borrowed("lang1"), Cow::Borrowed("lang2")],
                output: Cow::Borrowed("ComparisonResult"),
            },
            Capability {
                name: Cow::Borrowed("numeral_value"),
                description: Cow::Borrowed(
                    "Convert script numerals to numeric values (Greek isopsephy, Devanagari, etc.)",
                ),
                inputs: vec![Cow::Borrowed("text"), Cow::Borrowed("system")],
                output: Cow::Borrowed("u32"),
            },
        ],
        supported_languages: lang_codes,
        supported_scripts: script_codes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registration() {
        let reg = registration();
        assert_eq!(reg.name, "lipi");
        assert!(!reg.version.is_empty());
        assert!(!reg.capabilities.is_empty());
        assert!(reg.supported_languages.iter().any(|l| l == "en"));
        assert!(reg.supported_scripts.iter().any(|s| s == "Latn"));
    }

    #[test]
    fn test_registration_capabilities_count() {
        let reg = registration();
        assert_eq!(reg.capabilities.len(), 6);
    }

    #[test]
    fn test_registration_serde_roundtrip() {
        let reg = registration();
        let json = serde_json::to_string(&reg).unwrap();
        let back: AgentRegistration = serde_json::from_str(&json).unwrap();
        assert_eq!(reg, back);
    }

    #[test]
    fn test_registration_version_matches_cargo() {
        let reg = registration();
        assert_eq!(reg.version.as_ref(), env!("CARGO_PKG_VERSION"));
    }
}
