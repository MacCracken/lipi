# Security Policy

## Scope

Lipi is a pure language data library providing phoneme inventories, writing system metadata, grammar profiles, and lexicon access for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| String processing | Unicode edge cases, normalization | Standard Rust string handling; no raw pointer ops |
| Serde deserialization | Crafted JSON | Enum validation via serde derive |
| Phoneme lookup | Linear scan on large inventories | Bounded by inventory size (~50 phonemes per language) |
| Lexicon search | Large word lists | Consumer responsibility for input bounds |
| AI client (opt-in) | Network I/O to daimon/hoosh | Feature-gated; not compiled by default |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal core deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No `unwrap()` or `panic!()` in library code — all errors via `Result`
- All public types are `Send + Sync` (compile-time verified)
- No network I/O in core library (AI client is opt-in via feature flag)
- Minimal dependency surface (core depends only on serde, thiserror, tracing)
