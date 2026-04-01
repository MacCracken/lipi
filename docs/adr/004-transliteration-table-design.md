# ADR-004: Forward-Map Transliteration Tables

## Status

Accepted

## Context

Transliteration maps characters between scripts (e.g., Devanagari → IAST, Greek → Beta Code). Design options:

1. **HashMap bidirectional** — fast O(1) lookup both directions, higher memory
2. **Vec of pairs + linear scan** — simple, cache-friendly for small tables
3. **Trie** — optimal for multi-character greedy matching, complex implementation
4. **Regex-based** — flexible but heavy dependency

## Decision

Use `Vec<(Cow<'static, str>, Cow<'static, str>)>` for forward mappings with linear scan. Reverse maps are constructed on-demand via `reverse_map()` which returns a `HashMap`. Greedy longest-match transliteration tries up to 4 characters at each position.

## Consequences

**Benefits:**
- Zero heap allocation for static tables (all `Cow::Borrowed`)
- Simple, auditable implementation (~80 lines for `transliterate()`)
- Forward map preserves insertion order (useful for display/documentation)
- Reverse map is opt-in — only built when needed
- 4-char greedy match handles multi-codepoint sequences (e.g., "𒌋𒌋" → 20 in Babylonian)

**Trade-offs:**
- O(n) lookup per character in the forward direction (n = mapping count)
- For tables with 50+ mappings (Devanagari IAST has ~58), a HashMap would be faster
- The 4-char greedy limit is hardcoded — scripts with longer multi-char sequences would need adjustment
- `reverse_map()` loses information when multiple sources map to the same target (last wins)

**Future considerations:**
- When tables grow past ~100 entries (e.g., full Unicode transliteration), consider switching to a `HashMap` or `BTreeMap` for forward lookups
- The greedy limit should be configurable or computed from the table's max source length
