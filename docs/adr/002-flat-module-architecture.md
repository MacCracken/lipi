# ADR-002: Flat Module Architecture

## Status

Accepted

## Context

Lipi provides six distinct domains of linguistic data (phoneme, script, grammar, lexicon, registry, dialect). The module structure could be:

1. **Deeply nested** — `lipi::language::phoneme::inventory::english`
2. **Flat with submodules** — `lipi::phoneme::english()`, `lipi::script::by_code("Latn")`
3. **Single module** — everything in `lib.rs`

## Decision

Use a flat structure with one top-level module per domain. Submodules are used within a domain only when they represent a genuinely distinct concern (e.g., `phoneme::allophone`, `phoneme::syllable`, `script::transliteration`, `script::numerals`, `lexicon::swadesh`, `lexicon::cognate`).

```
src/
  lib.rs          — re-exports, crate docs
  error.rs        — LipiError
  registry.rs     — central lookup
  phoneme/
    mod.rs         — core types + pre-built inventories
    inventories.rs — extended language inventories
    allophone.rs   — allophone rules
    syllable.rs    — phonotactic constraints
  script/
    mod.rs         — Script type + pre-built scripts
    transliteration.rs
    numerals.rs
  grammar/mod.rs
  lexicon/
    mod.rs
    swadesh.rs
    cognate.rs
  dialect.rs
```

## Consequences

**Benefits:**
- Import paths are short: `lipi::phoneme::english()`, `lipi::script::by_code("Deva")`
- Each domain is independently navigable
- Adding a new domain (e.g., `prosody`) requires only a new top-level module
- Consumers pull exactly the domain they need

**Trade-offs:**
- Some modules (`phoneme/inventories.rs`) are large (3000+ lines) because they contain data for many languages
- Cross-module queries (e.g., "all data for language X") go through `registry` rather than a unified per-language struct
