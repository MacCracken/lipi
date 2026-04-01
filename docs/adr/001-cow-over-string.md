# ADR-001: Cow<'static, str> over String for Static Data

## Status

Accepted

## Context

Lipi stores large amounts of static linguistic data — IPA symbols, language names, script codes, Swadesh words, transliteration mappings — that are known at compile time. The choice of string type for these fields affects both memory allocation behavior and API ergonomics.

Options considered:
1. **`String`** — always heap-allocated, simple API
2. **`&'static str`** — zero-cost for static data, but prevents runtime construction
3. **`Cow<'static, str>`** — borrows static data, allocates only when needed

## Decision

Use `Cow<'static, str>` for all string fields in public structs (`Phoneme`, `PhonemeInventory`, `Script`, `LexEntry`, `GrammarProfile`, etc.).

## Consequences

**Benefits:**
- Pre-built inventories (51 languages) use `Cow::Borrowed`, producing zero heap allocations
- The builder pattern and runtime construction still work via `Cow::Owned`
- Serde derives work naturally with Cow
- Consumers that only read static data pay zero allocation cost

**Trade-offs:**
- Slightly more complex type signatures than plain `String`
- `impl Into<Cow<'static, str>>` on builder methods adds generic complexity
- Clone of `Cow::Borrowed` is free, but clone of `Cow::Owned` still allocates

**Validation:**
- Benchmark: `english_phoneme_inventory` builds 36 phonemes with zero heap allocations for IPA strings
- All pre-built functions (`english()`, `sanskrit()`, etc.) use only `Cow::Borrowed`
