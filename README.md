# Lipi

> **Lipi** (Sanskrit: लिपि — script, writing system) — multilingual language engine for AGNOS

Structured, queryable corpus of human language data. Phoneme inventories, writing system metadata, grammar profiles, and lexicon access for 50+ languages.

Used by [shabda](https://github.com/MacCracken/shabda) (G2P conversion), [shabdakosh](https://github.com/MacCracken/shabdakosh) (pronunciation dictionary), [svara](https://github.com/MacCracken/svara) (vocal synthesis), [jnana](https://github.com/MacCracken/jnana) (knowledge system), and [vidya](https://github.com/MacCracken/vidya) (programming reference).

## Modules

| Module | Description |
|--------|-------------|
| `phoneme` | IPA phoneme inventories per language, articulatory features (manner, place, voicing), stress/tone patterns |
| `script` | Writing system metadata: alphabet, syllabary, logographic, abjad, abugida. Unicode ranges, directionality |
| `grammar` | Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order, case systems |
| `lexicon` | Core vocabulary per language (Swadesh lists, frequency-ranked word lists), cognate detection |

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support |
| `logging` | no | Structured logging via `LIPI_LOG` env var |
| `full` | -- | Enables all features |

## Quick Start

```toml
[dependencies]
lipi = "0.1"
```

```rust
use lipi::phoneme::{self, StressPattern};

// Get the English phoneme inventory
let en = phoneme::english();
assert!(en.has("θ"));           // voiceless dental fricative (think)
assert!(en.has("ð"));           // voiced dental fricative (this)
assert!(!en.has("ʀ"));          // no uvular trill in English
assert_eq!(en.stress, StressPattern::Free);
println!("{} consonants, {} vowels", en.consonant_count(), en.vowel_count());

// Look up a specific phoneme
let sh = en.find("ʃ").unwrap(); // postalveolar fricative (ship)
```

## Architecture

```text
lipi (this) — language structure & phoneme inventories
  | provides phoneme sets per language
shabda — G2P conversion (currently English-only, lipi makes it multilingual)
  | produces phoneme sequences
shabdakosh — pronunciation dictionary (currently CMUdict, lipi adds IPA dicts)
  | lookup fallback
svara — vocal synthesis (consumes phonemes, produces audio)
  | voice output
dhvani — audio engine (mixing, DSP, output)
```

Also feeds:
- **jnana** — multilingual knowledge access
- **vidya** — programming concepts explained in native languages
- **vansh** (planned) — voice assistant with multilingual TTS/STT
- **sahifa** (planned) — OCR language detection, multilingual document processing

## Development

```bash
make check     # fmt + clippy + test + audit
make bench     # Run benchmarks with history tracking
make coverage  # Generate coverage report
make doc       # Build documentation
```

## License

GPL-3.0-only. See [LICENSE](LICENSE).
