# Development Roadmap

> **Status**: Pre-1.0 | **Current**: 0.5.0
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

### 0.1.0 — Scaffold Hardening (2026-03-31)

- [x] Cow<'static, str> migration for zero-alloc static inventories
- [x] PartialEq/Eq derives on all public types
- [x] #[non_exhaustive] on PhonemeKind variants with Phoneme::consonant/vowel constructors
- [x] LabialVelar place of articulation, /w/ reclassified
- [x] Tracing instrumentation on public methods
- [x] Expanded test coverage (29 tests)
- [x] Cargo.lock removed from tracking

### 0.2.0 — Sankhya Foundation & Script Registry (2026-03-31)

- [x] `[S]` Sanskrit phoneme inventory (36 consonants + 15 vowels, 5 vargas for Katapayadi)
- [x] `[S]` Greek phoneme inventory (20 consonants + 5 vowels)
- [x] Script metadata for: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana
- [x] `[S]` Script metadata for: Greek alphabet (Unicode range, directionality)
- [x] Builder pattern for PhonemeInventory construction (`PhonemeInventoryBuilder`)
- [x] Language registry with ISO 639 lookup (`registry` module)

### 0.3.0 — Allophone & Phonotactics (2026-03-31)

- [x] Allophone rules per language (`phoneme::allophone` — Environment, PhonemeClass, AllophoneRule, AllophoneRuleSet)
- [x] Phonotactic constraints (`phoneme::syllable` — PhonotacticConstraint, ConstraintKind, Phonotactics)
- [x] Syllable structure templates (SyllableTemplate — onset/nucleus/coda, English/Sanskrit/Japanese profiles)
- [x] `[S]` Romanization/transliteration tables (`script::transliteration` — Devanagari↔IAST, Greek↔Beta Code)
- [x] `[S]` Script-to-numeral mapping API (`script::numerals` — Devanagari digits, Greek isopsephy)

### 0.4.0 — Extended Coverage (2026-03-31)

- [x] `[S]` Yucatec Maya phoneme inventory (21C + 10V, ejectives for Mayan calendar)
- [x] 11 additional language inventories: Swahili, Yoruba, Zulu, Thai, Vietnamese, Tagalog, Turkish, Finnish, Hawaiian, Nahuatl (14 total languages)
- [x] Dialect/variety support (`dialect` module — LanguageVariety, VarietyKind, phoneme overlays)
- [x] Cognate detection types (`lexicon::cognate` — CognateSet, CognateEntry, proto-forms)
- [x] Loanword tracking and etymology (`Etymology`, `BorrowingType`)

### 0.4.1 — Classical & Ancient Scripts (2026-03-31)

- [x] `[S]` Cuneiform script metadata (Xsux) + Babylonian sexagesimal numeral system
- [x] `[S]` Egyptian hieroglyphic script metadata (Egyp) + additive decimal numeral system
- [x] `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- [x] Classical/Liturgical language profiles: Latin, Classical Arabic, Koine Greek, Literary Chinese (Sanskrit already in 0.2.0)
- [x] Dead script classification: `ScriptStatus` (Living/Limited/Historical) + attestation periods

### 0.5.0 — Core Languages (2026-03-31)

- [x] Language inventories: Mandarin, Hindi, Japanese, Spanish, French, German, Russian, Korean, Portuguese (Arabic in 0.4.1)
- [x] Grammar profiles for all 10 core languages (`grammar::by_code()`)
- [x] Swadesh-25 starter lists for each language (`lexicon::swadesh::by_code()`, 250 entries)

## Backlog

### 0.6.0 — AI Integration

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
