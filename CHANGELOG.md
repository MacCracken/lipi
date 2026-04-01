# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-03-31

### Added

- **phoneme::inventories** — 9 core language inventories: Mandarin (21C+7V), Hindi (34C+10V), Japanese (20C+5V), Spanish (23C+5V), French (21C+16V), German (23C+16V), Russian (36C+6V), Korean (19C+7V), Portuguese (23C+14V)
- **grammar** — Pre-built grammar profiles for 10 core languages with `by_code()` lookup and `all_codes()`. Covers morphology, word order, case count, gender, dual number, classifiers
- **lexicon::swadesh** — Swadesh-25 starter word lists for 10 core languages (250 entries total) with IPA transcription, part of speech, and Swadesh index. `by_code()` lookup
- **registry** — 27 languages registered (up from 18)

## [0.4.1] - 2026-03-31

### Added

- **script** — `[S]` Cuneiform script metadata (Xsux) with Sumerian/Akkadian language tags and Unicode ranges
- **script** — `[S]` Egyptian hieroglyphic script metadata (Egyp) with Unicode ranges
- **script** — `ScriptStatus` enum (Living, Limited, Historical) and `attestation` period field on all scripts
- **script::numerals** — `[S]` Babylonian sexagesimal numeral system (base-60, cuneiform digits)
- **script::numerals** — `[S]` Egyptian hieroglyphic numeral system (additive decimal, powers of 10)
- **script::numerals** — `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- **phoneme::inventories** — 4 classical/liturgical language inventories:
  - **Latin** (la) — 18C + 10V, labialized velars
  - **Classical Arabic** (ar) — 28C + 6V, pharyngeals, emphatics, uvular
  - **Koine Greek** (grc) — 17C + 5V, pitch accent
  - **Literary Chinese** (lzh) — 27C + 12V, 4-tone Middle Chinese reconstruction
- **registry** — 18 languages registered (up from 14), 10 scripts (up from 8)

## [0.4.0] - 2026-03-31

### Added

- **dialect** — Language variety support: `LanguageVariety`, `VarietyKind` (Regional, NationalStandard, Historical, Sociolect, Creole). Phoneme add/remove overlays and allophone overrides. British English (RP) and Egyptian Arabic pre-built
- **lexicon::cognate** — Cognate detection types: `CognateSet` with proto-form and cross-language entries, `Etymology` with `BorrowingType` (Loanword, Calque, SemanticLoan, Inherited). PIE "water" cognate set pre-built
- **phoneme::inventories** — 11 new language inventories across 6 language families:
  - `[S]` **Yucatec Maya** (yua) — 21C + 10V, ejective consonants for Mayan calendar validation
  - **Swahili** (sw) — 26C + 5V, **Yoruba** (yo) — 18C + 7V (3-tone), **Zulu** (zu) — 42C + 5V (clicks)
  - **Thai** (th) — 21C + 9V (5-tone), **Vietnamese** (vi) — 22C + 11V (6-tone), **Tagalog** (tl) — 18C + 5V
  - **Turkish** (tr) — 20C + 8V (vowel harmony), **Finnish** (fi) — 17C + 16V (short+long)
  - **Hawaiian** (haw) — 8C + 10V (minimal inventory), **Nahuatl** (nah) — 16C + 8V (lateral affricate)
- **registry** — 14 languages registered (up from 3)

## [0.3.0] - 2026-03-31

### Added

- **phoneme::allophone** — Allophone rule system: `AllophoneRuleSet`, `AllophoneRule`, `Environment`, `PhonemeClass`. Context-dependent sound variation with `rules_for()` and `realize()` lookup. English (GA) rules included (aspiration, flapping, dark-l)
- **phoneme::syllable** — Syllable structure templates: `SyllableTemplate` with max onset/coda, `Phonotactics` with `PhonotacticConstraint`. English, Sanskrit, and Japanese profiles included
- **script::transliteration** — `[S]` Bidirectional transliteration tables: `TransliterationTable` with `transliterate()`, `transliterate_char()`, and `reverse_map()`. Devanagari↔IAST and Greek↔Beta Code tables included
- **script::numerals** — `[S]` Script-to-numeral mapping: `NumeralSystem` with `value_of()`, `char_for()`, and `string_value()`. Devanagari decimal digits and Greek isopsephy included
- `Hash` derive on `Phoneme`, `PhonemeKind`, `Morphology`, `WordOrder`, `Direction`
- `debug_assert` for duplicate IPA detection in `PhonemeInventoryBuilder::build()`

### Changed

- **script** — Kana script name corrected from "Katakana" to "Kana (Hiragana + Katakana)"
- **registry** — `all_codes()` returns `&'static [&'static str]` instead of allocating `Vec`
- **lib.rs** — Crate documentation updated to reflect five modules (added registry)
- **docs** — Architecture overview rewritten with registry, script/language tables

## [0.2.0] - 2026-03-31

### Added

- **phoneme** — `PhonemeInventoryBuilder` for ergonomic inventory construction with `consonant()`, `vowel()`, `stress()`, `tones()`, and `with_capacity()` methods
- **phoneme** — Sanskrit (Classical) phoneme inventory: 36 consonants + 15 vowels, organized by 5 vargas (consonant groups for Katapayadi encoding) `[S]`
- **phoneme** — Greek (Modern Standard) phoneme inventory: 20 consonants + 5 vowels `[S]`
- **script** — Pre-built metadata for 8 writing systems: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana, Greek `[S]`
- **script** — `by_code()` lookup by ISO 15924 code, `all_codes()` enumeration, `Script::contains_codepoint()` for Unicode range checking
- **registry** — Language registry with ISO 639 lookup: `info()`, `phonemes()`, `primary_script()`, `all_codes()`
- **phoneme** — `LabialVelar` place of articulation variant
- **phoneme** — `Phoneme::consonant()` and `Phoneme::vowel()` constructors for external use
- **phoneme** — `PartialEq`/`Eq` derives on `Phoneme`, `PhonemeKind`, `PhonemeInventory`
- **script** — `PartialEq`/`Eq` derives on `Script`
- **grammar** — `PartialEq`/`Eq` derives on `GrammarProfile`
- **lexicon** — `PartialEq`/`Eq` derives on `LexEntry`, `Lexicon`
- 6 criterion benchmarks (3 inventories, phoneme lookup, registry lookup, script lookup)

### Changed

- **phoneme** — All string fields migrated to `Cow<'static, str>` for zero-alloc static inventories
- **phoneme** — English `/w/` reclassified from `Bilabial` to `LabialVelar` (linguistically accurate)
- **phoneme** — `english()` refactored to use `PhonemeInventoryBuilder`
- **phoneme** — `#[non_exhaustive]` added to `PhonemeKind::Consonant` and `PhonemeKind::Vowel` variants
- **script** — All string fields migrated to `Cow<'static, str>`
- **grammar** — `language_code` migrated to `Cow<'static, str>`
- **lexicon** — All string fields migrated to `Cow<'static, str>`
- Tracing instrumentation added to all public lookup methods

### Performance

- `english_phoneme_inventory`: 146ns (builder with pre-alloc)
- `sanskrit_phoneme_inventory`: 197ns
- `greek_phoneme_inventory`: 90ns
- `phoneme_lookup_ipa`: 14ns
- `registry_phonemes_lookup`: 200ns
- `script_by_code_lookup`: 19ns

## [0.1.0] - 2026-03-30

### Added

- **phoneme** — IPA phoneme inventories with articulatory features (manner, place, voicing, height, backness, rounding), stress patterns, tone systems. English (General American) inventory included
- **script** — Writing system metadata: alphabet, syllabary, logographic, abjad, abugida, mixed. Unicode ranges, directionality (LTR, RTL, TTB, bidi)
- **grammar** — Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order (SVO/SOV/VSO/VOS/OVS/OSV/Free), case systems, gender, dual number, classifiers
- **lexicon** — Lexical entries with IPA transcription, part of speech, frequency ranking, Swadesh list indexing. Lookup, Swadesh extraction, frequency ranking
- **error** — `LipiError` with variants for unknown language/script, missing phonemes, invalid IPA, word-not-found
- **logging** — Optional structured logging via `LIPI_LOG` env var (feature-gated)
- Initial criterion benchmarks for phoneme inventory construction and lookup
