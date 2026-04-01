# ADR-005: Feature-Gated AI Integration Modules

## Status

Accepted

## Context

Lipi serves as a data layer for downstream crates (shabda, svara, jnana) but also integrates with AGNOS AI infrastructure (MCP tools, daimon agent framework, hoosh LLM queries). These integrations:

- Add a dependency on `serde_json` (not needed for core linguistic data)
- Define AI-specific types and protocols
- Are only relevant to consumers that use the AGNOS AI stack

Options:
1. **Always included** — simpler, but bloats minimal consumers
2. **Separate crates** — `lipi-mcp`, `lipi-daimon` — maximum isolation, more repos
3. **Feature-gated modules** — opt-in at compile time, single crate

## Decision

Feature-gate AI integration modules behind `mcp`, `daimon`, and `hoosh` features. All three depend on `std` and `serde_json`. A `full` feature enables everything.

```toml
[features]
default = ["std"]
std = ["serde/std", "thiserror/std"]
full = ["std", "logging", "mcp", "daimon", "hoosh"]
mcp = ["std", "dep:serde_json"]
daimon = ["std", "dep:serde_json"]
hoosh = ["std", "dep:serde_json"]
```

## Consequences

**Benefits:**
- Core library (`default` features) has only 3 dependencies: serde, thiserror, tracing
- `no_std` consumers (embedded, WASM) can use phoneme/script/grammar data without pulling in serde_json
- Each AI integration can evolve independently
- `full` feature provides a one-line opt-in for AGNOS stack consumers

**Trade-offs:**
- Feature combinations must be tested in CI (default, individual features, full)
- Conditional compilation (`#[cfg(feature = "mcp")]`) adds complexity
- Documentation must clarify which types are available under which features

**Validation:**
- `cargo check --no-default-features` passes (core is no_std-compatible with alloc)
- `cargo check --features mcp` enables only MCP tools
- `cargo test --all-features` exercises everything
