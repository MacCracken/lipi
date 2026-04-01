# Architecture Overview

> **Lipi** — multilingual language engine

## Module Map

```
lipi/
├── src/
│   ├── lib.rs            — public API, module re-exports
│   ├── error.rs          — LipiError enum (non_exhaustive)
│   ├── phoneme/mod.rs    — IPA inventories, articulatory features, stress/tone
│   │                       PhonemeInventoryBuilder, english/sanskrit/greek
│   ├── script/mod.rs     — writing systems, Unicode ranges, directionality
│   │                       8 pre-built scripts, by_code() lookup
│   ├── grammar/mod.rs    — morphology, word order, case systems
│   ├── lexicon/mod.rs    — vocabulary, Swadesh lists, frequency ranking
│   ├── registry.rs       — language registry, ISO 639 lookup
│   └── logging.rs        — optional LIPI_LOG env-based tracing init
├── benches/
│   └── benchmarks.rs     — criterion benchmarks (6 benchmarks)
├── tests/
│   └── integration.rs    — cross-module integration tests
└── examples/
    └── basic.rs          — runnable usage example
```

## Data Flow

```
Language selection (ISO 639 code)
  │
  ├─→ registry  — central lookup: phonemes(), primary_script(), info()
  │     │
  │     ├─→ phoneme — IPA inventory, consonant/vowel counts, stress pattern
  │     └─→ script  — writing system type, direction, Unicode ranges
  │
  ├─→ grammar — morphology, word order, case/gender/number systems
  └─→ lexicon — word lookup, Swadesh list, frequency ranking
```

## Registered Languages

| Code | Language | Phonemes | Script |
|------|----------|----------|--------|
| en   | English  | 24C + 12V | Latin  |
| sa   | Sanskrit | 36C + 14V | Devanagari |
| el   | Greek    | 20C + 5V  | Greek  |

## Registered Scripts

| Code | Name | Type | Direction |
|------|------|------|-----------|
| Latn | Latin | Alphabet | LTR |
| Arab | Arabic | Abjad | RTL |
| Deva | Devanagari | Abugida | LTR |
| Hani | CJK Ideographs | Logographic | LTR |
| Cyrl | Cyrillic | Alphabet | LTR |
| Hang | Hangul | Alphabet | LTR |
| Kana | Kana (Hiragana + Katakana) | Syllabary | LTR |
| Grek | Greek | Alphabet | LTR |

## Dependency Stack

```
lipi (this crate)
  │
  ├── serde      — serialization for all types
  ├── thiserror  — error derivation
  └── tracing    — structured logging
```

## Downstream Consumers

```
lipi
  ├─→ shabda      — G2P conversion (multilingual phoneme sets)
  ├─→ shabdakosh  — pronunciation dictionary (IPA dictionaries)
  ├─→ svara       — vocal synthesis (phoneme-to-audio)
  ├─→ sankhya     — ancient mathematical systems (script-aware numerals) [S]
  ├─→ jnana       — multilingual knowledge access
  ├─→ vidya       — programming reference (native language explanations)
  ├─→ vansh       — voice assistant TTS/STT (planned)
  └─→ sahifa      — OCR language detection (planned)
```

## Design Principles

- **Data-driven**: Language data as structured Rust types, not embedded strings
- **Queryable**: Every inventory supports lookup, filtering, counting
- **Composable**: Each module is independent — consumers pull only what they need
- **Serializable**: All types implement Serialize + Deserialize for data exchange
- **Extensible**: `#[non_exhaustive]` on all enums — new variants without breaking changes
- **Zero-alloc statics**: `Cow<'static, str>` for pre-built inventories
- **Builder pattern**: `PhonemeInventoryBuilder` for ergonomic inventory construction
