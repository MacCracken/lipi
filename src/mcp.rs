//! MCP (Model Context Protocol) tool definitions for AI agent integration.
//!
//! Exposes lipi's language data as structured tools that AI agents can
//! invoke via MCP. Each tool has a name, description, input parameters,
//! and a handler that returns JSON-serializable output.

use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Description of an MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name (e.g., "lipi_phonemes").
    pub name: Cow<'static, str>,
    /// Human-readable description.
    pub description: Cow<'static, str>,
    /// Parameter descriptions: name → (type, description).
    pub parameters: Vec<ParameterDef>,
}

/// A tool parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    /// Parameter name.
    pub name: Cow<'static, str>,
    /// JSON Schema type (e.g., "string", "integer").
    pub param_type: Cow<'static, str>,
    /// Description.
    pub description: Cow<'static, str>,
    /// Whether this parameter is required.
    pub required: bool,
}

/// Result of an MCP tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ToolResult {
    /// Successful result with JSON-serializable data.
    Success(serde_json::Value),
    /// Error with message.
    Error(String),
}

/// All MCP tool definitions exposed by lipi.
#[must_use]
pub fn tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: Cow::Borrowed("lipi_phonemes"),
            description: Cow::Borrowed(
                "Get the phoneme inventory for a language. Returns consonants, vowels, \
                 stress pattern, and tone system.",
            ),
            parameters: vec![ParameterDef {
                name: Cow::Borrowed("language"),
                param_type: Cow::Borrowed("string"),
                description: Cow::Borrowed("ISO 639 language code (e.g., 'en', 'ja', 'ar')"),
                required: true,
            }],
        },
        ToolDefinition {
            name: Cow::Borrowed("lipi_script"),
            description: Cow::Borrowed(
                "Get writing system metadata for a script. Returns type, direction, \
                 Unicode ranges, and status.",
            ),
            parameters: vec![ParameterDef {
                name: Cow::Borrowed("code"),
                param_type: Cow::Borrowed("string"),
                description: Cow::Borrowed("ISO 15924 script code (e.g., 'Latn', 'Arab', 'Deva')"),
                required: true,
            }],
        },
        ToolDefinition {
            name: Cow::Borrowed("lipi_grammar"),
            description: Cow::Borrowed(
                "Get the grammatical profile for a language. Returns morphology type, \
                 word order, case count, gender, and classifier usage.",
            ),
            parameters: vec![ParameterDef {
                name: Cow::Borrowed("language"),
                param_type: Cow::Borrowed("string"),
                description: Cow::Borrowed("ISO 639 language code (e.g., 'de', 'ja', 'ru')"),
                required: true,
            }],
        },
        ToolDefinition {
            name: Cow::Borrowed("lipi_translate_ipa"),
            description: Cow::Borrowed(
                "Transliterate text between scripts. Supports Devanagari↔IAST \
                 and Greek↔Beta Code.",
            ),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("text"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Text to transliterate"),
                    required: true,
                },
                ParameterDef {
                    name: Cow::Borrowed("scheme"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed(
                        "Transliteration scheme: 'devanagari_iast' or 'greek_beta'",
                    ),
                    required: true,
                },
            ],
        },
        ToolDefinition {
            name: Cow::Borrowed("lipi_compare"),
            description: Cow::Borrowed(
                "Compare two languages. Returns shared/unique phonemes, typological \
                 differences, and cognate information.",
            ),
            parameters: vec![
                ParameterDef {
                    name: Cow::Borrowed("lang1"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("First language ISO 639 code"),
                    required: true,
                },
                ParameterDef {
                    name: Cow::Borrowed("lang2"),
                    param_type: Cow::Borrowed("string"),
                    description: Cow::Borrowed("Second language ISO 639 code"),
                    required: true,
                },
            ],
        },
    ]
}

/// Invoke an MCP tool by name with the given parameters.
#[must_use]
pub fn invoke(tool_name: &str, params: &HashMap<String, String>) -> ToolResult {
    tracing::info!(tool = tool_name, "MCP tool invocation");
    match tool_name {
        "lipi_phonemes" => handle_phonemes(params),
        "lipi_script" => handle_script(params),
        "lipi_grammar" => handle_grammar(params),
        "lipi_translate_ipa" => handle_transliterate(params),
        "lipi_compare" => handle_compare(params),
        _ => ToolResult::Error(format!("unknown tool: {tool_name}")),
    }
}

fn get_param<'a>(params: &'a HashMap<String, String>, key: &str) -> Result<&'a str, ToolResult> {
    params
        .get(key)
        .map(|s| s.as_str())
        .ok_or_else(|| ToolResult::Error(format!("missing required parameter: {key}")))
}

fn handle_phonemes(params: &HashMap<String, String>) -> ToolResult {
    let lang = match get_param(params, "language") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match crate::registry::phonemes(lang) {
        Some(inv) => match serde_json::to_value(&inv) {
            Ok(v) => ToolResult::Success(v),
            Err(e) => ToolResult::Error(format!("serialization error: {e}")),
        },
        None => ToolResult::Error(format!("unknown language: {lang}")),
    }
}

fn handle_script(params: &HashMap<String, String>) -> ToolResult {
    let code = match get_param(params, "code") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match crate::script::by_code(code) {
        Some(script) => match serde_json::to_value(&script) {
            Ok(v) => ToolResult::Success(v),
            Err(e) => ToolResult::Error(format!("serialization error: {e}")),
        },
        None => ToolResult::Error(format!("unknown script: {code}")),
    }
}

fn handle_grammar(params: &HashMap<String, String>) -> ToolResult {
    let lang = match get_param(params, "language") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match crate::grammar::by_code(lang) {
        Some(grammar) => match serde_json::to_value(&grammar) {
            Ok(v) => ToolResult::Success(v),
            Err(e) => ToolResult::Error(format!("serialization error: {e}")),
        },
        None => ToolResult::Error(format!("no grammar profile for: {lang}")),
    }
}

fn handle_transliterate(params: &HashMap<String, String>) -> ToolResult {
    let text = match get_param(params, "text") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let scheme = match get_param(params, "scheme") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let table = match scheme {
        "devanagari_iast" => crate::script::transliteration::devanagari_iast(),
        "greek_beta" => crate::script::transliteration::greek_beta_code(),
        _ => return ToolResult::Error(format!("unknown scheme: {scheme}")),
    };
    let result = table.transliterate(text);
    ToolResult::Success(serde_json::Value::String(result))
}

fn handle_compare(params: &HashMap<String, String>) -> ToolResult {
    let lang1 = match get_param(params, "lang1") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let lang2 = match get_param(params, "lang2") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let inv1 = match crate::registry::phonemes(lang1) {
        Some(i) => i,
        None => return ToolResult::Error(format!("unknown language: {lang1}")),
    };
    let inv2 = match crate::registry::phonemes(lang2) {
        Some(i) => i,
        None => return ToolResult::Error(format!("unknown language: {lang2}")),
    };

    let ipas1: std::collections::HashSet<&str> =
        inv1.phonemes.iter().map(|p| p.ipa.as_ref()).collect();
    let ipas2: std::collections::HashSet<&str> =
        inv2.phonemes.iter().map(|p| p.ipa.as_ref()).collect();

    let shared: Vec<&str> = ipas1.intersection(&ipas2).copied().collect();
    let only1: Vec<&str> = ipas1.difference(&ipas2).copied().collect();
    let only2: Vec<&str> = ipas2.difference(&ipas1).copied().collect();

    let mut result = serde_json::Map::new();
    result.insert("lang1".into(), serde_json::Value::String(lang1.into()));
    result.insert("lang2".into(), serde_json::Value::String(lang2.into()));
    result.insert(
        "shared_phonemes".into(),
        serde_json::to_value(&shared).unwrap_or_default(),
    );
    result.insert(
        "unique_to_lang1".into(),
        serde_json::to_value(&only1).unwrap_or_default(),
    );
    result.insert(
        "unique_to_lang2".into(),
        serde_json::to_value(&only2).unwrap_or_default(),
    );
    result.insert(
        "shared_count".into(),
        serde_json::Value::Number(shared.len().into()),
    );

    // Add grammar comparison if available
    if let (Some(g1), Some(g2)) = (
        crate::grammar::by_code(lang1),
        crate::grammar::by_code(lang2),
    ) {
        let mut grammar = serde_json::Map::new();
        grammar.insert(
            "same_word_order".into(),
            serde_json::Value::Bool(g1.word_order == g2.word_order),
        );
        grammar.insert(
            "same_morphology".into(),
            serde_json::Value::Bool(g1.morphology == g2.morphology),
        );
        result.insert(
            "grammar_comparison".into(),
            serde_json::Value::Object(grammar),
        );
    }

    ToolResult::Success(serde_json::Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_definitions_count() {
        let tools = tool_definitions();
        assert_eq!(tools.len(), 5);
    }

    #[test]
    fn test_tool_names() {
        let tools = tool_definitions();
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(names.contains(&"lipi_phonemes"));
        assert!(names.contains(&"lipi_script"));
        assert!(names.contains(&"lipi_grammar"));
        assert!(names.contains(&"lipi_translate_ipa"));
        assert!(names.contains(&"lipi_compare"));
    }

    #[test]
    fn test_invoke_phonemes() {
        let mut params = HashMap::new();
        params.insert("language".into(), "en".into());
        match invoke("lipi_phonemes", &params) {
            ToolResult::Success(v) => {
                assert!(v.get("language_code").is_some());
            }
            ToolResult::Error(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_invoke_phonemes_unknown() {
        let mut params = HashMap::new();
        params.insert("language".into(), "xx".into());
        assert!(matches!(
            invoke("lipi_phonemes", &params),
            ToolResult::Error(_)
        ));
    }

    #[test]
    fn test_invoke_script() {
        let mut params = HashMap::new();
        params.insert("code".into(), "Deva".into());
        match invoke("lipi_script", &params) {
            ToolResult::Success(v) => {
                assert_eq!(v.get("code").unwrap().as_str().unwrap(), "Deva");
            }
            ToolResult::Error(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_invoke_grammar() {
        let mut params = HashMap::new();
        params.insert("language".into(), "de".into());
        match invoke("lipi_grammar", &params) {
            ToolResult::Success(v) => {
                assert_eq!(v.get("case_count").unwrap().as_u64().unwrap(), 4);
            }
            ToolResult::Error(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_invoke_transliterate() {
        let mut params = HashMap::new();
        params.insert("text".into(), "αβγ".into());
        params.insert("scheme".into(), "greek_beta".into());
        match invoke("lipi_translate_ipa", &params) {
            ToolResult::Success(v) => {
                assert_eq!(v.as_str().unwrap(), "abg");
            }
            ToolResult::Error(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_invoke_compare() {
        let mut params = HashMap::new();
        params.insert("lang1".into(), "es".into());
        params.insert("lang2".into(), "de".into());
        match invoke("lipi_compare", &params) {
            ToolResult::Success(v) => {
                assert!(v.get("shared_phonemes").is_some());
                assert!(v.get("shared_count").is_some());
                assert!(v.get("grammar_comparison").is_some());
            }
            ToolResult::Error(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn test_invoke_unknown_tool() {
        let params = HashMap::new();
        assert!(matches!(
            invoke("nonexistent", &params),
            ToolResult::Error(_)
        ));
    }

    #[test]
    fn test_invoke_missing_param() {
        let params = HashMap::new();
        assert!(matches!(
            invoke("lipi_phonemes", &params),
            ToolResult::Error(_)
        ));
    }

    #[test]
    fn test_tool_definitions_serde() {
        let tools = tool_definitions();
        let json = serde_json::to_string(&tools).unwrap();
        let back: Vec<ToolDefinition> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 5);
    }
}
