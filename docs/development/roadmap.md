# Development Roadmap

> **Status**: Pre-1.0 | **Current**: 0.1.0
>
> Items marked `[S]` also unblock **sankhya** (ancient mathematical systems).

## Completed

### 0.1.0 — Scaffold (2026-03-30)

- [x] Core type system: Phoneme, Script, GrammarProfile, Lexicon, LexEntry
- [x] Articulatory features: Manner, Place, Height, Backness, voicing, rounding
- [x] Writing system classification: Alphabet, Abugida, Abjad, Syllabary, Logographic, Mixed
- [x] Grammar typology: Isolating, Agglutinative, Fusional, Polysynthetic
- [x] Word order variants: SVO, SOV, VSO, VOS, OVS, OSV, Free
- [x] Lexicon with Swadesh indexing and frequency ranking
- [x] English (General American) phoneme inventory
- [x] Error types with thiserror
- [x] Optional structured logging
- [x] Initial criterion benchmarks

## Backlog

### 0.2.0 — Core Languages

- [ ] Language inventories: Arabic, Mandarin, Hindi, Japanese, Spanish, French, German, Russian, Korean, Portuguese
- [ ] `[S]` Sanskrit phoneme inventory (consonant groups needed for Katapayadi encoding)
- [ ] `[S]` Greek phoneme inventory (alphabet metadata for Greek mathematical notation)
- [ ] Script metadata for: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana
- [ ] `[S]` Script metadata for: Greek alphabet (Unicode range, directionality)
- [ ] Grammar profiles for all 10 core languages
- [ ] Basic Swadesh lists (100-word) for each language
- [ ] Builder pattern for PhonemeInventory construction
- [ ] Language registry with ISO 639 lookup

### 0.3.0 — Allophone & Phonotactics

- [ ] Allophone rules per language (context-dependent sound variation)
- [ ] Phonotactic constraints (valid syllable structures)
- [ ] Syllable structure templates (onset-nucleus-coda)
- [ ] `[S]` Romanization/transliteration tables per script (Devanagari IAST/ISO 15919, Greek Beta Code)
- [ ] `[S]` Script-to-numeral mapping API (generic interface for scripts with numeric values)

### 0.4.0 — Extended Coverage

- [ ] 30+ additional languages (African, Southeast Asian, Indigenous American)
- [ ] `[S]` Yucatec Maya — phoneme inventory, transliteration conventions (day sign / month name validation for Mayan calendar)
- [ ] Dialect/variety support (e.g., British vs American English, MSA vs Egyptian Arabic)
- [ ] Cognate detection between related languages
- [ ] Loanword tracking and etymology markers

### 0.4.x — Classical & Ancient Scripts

- [ ] `[S]` Cuneiform script metadata — Sumerian/Akkadian (Babylonian sexagesimal numeral display)
- [ ] `[S]` Egyptian hieratic/hieroglyphic script metadata (stellar decan names, unit fraction notation)
- [ ] `[S]` Classical Chinese rod numeral script conventions (positional display rules)
- [ ] Classical/Liturgical language profiles: Sanskrit, Classical Arabic, Koine Greek, Literary Chinese, Latin
- [ ] Dead script classification and historical attestation metadata

### 0.5.0 — AI Integration

- [ ] Daimon client for agent registration
- [ ] Hoosh client for LLM-powered language queries
- [ ] MCP tools: `lipi_phonemes`, `lipi_script`, `lipi_grammar`, `lipi_translate_ipa`, `lipi_compare`

## Future (demand-gated)

- Prosody patterns (intonation contours, rhythm class)
- Morphological analyzer (stemming, lemmatization per language)
- Frequency corpus integration (Google Ngrams, OpenSubtitles)
- Historical phonology (sound change rules, Proto-IE reconstructions)
- Sign language phonology (handshape, location, movement features)

## v1.0 Criteria

- [ ] 50+ language inventories with verified phoneme data
- [ ] All modules have 80%+ test coverage
- [ ] Criterion benchmarks with 3-point trend history
- [ ] Full serde roundtrip tests for all public types
- [ ] shabda + shabdakosh consuming lipi for multilingual G2P
- [ ] `[S]` sankhya consuming lipi for script-aware numeral display and transliteration
- [ ] Documentation: architecture overview, usage guide, API docs
- [ ] Published on crates.io
