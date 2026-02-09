# Rhema Chirho

> *"For God so loved the world that he gave his only begotten Son,
> that whoever believes in him should not perish but have eternal life."*
> **-- John 3:16**

**Biblical scholarship engine in Pure Rust.**

Rhema Chirho is a multi-crate workspace providing a complete biblical
scholarship engine: query parsing, planning, indexing, execution,
discourse analysis, AI-powered semantic search, and integration surfaces
(CLI, GUI, REST API, WASM, FFI).

Built on [rsword_chirho](https://github.com/loveJesus/rsword-chirho) --
a Pure Rust port of the SWORD Bible software library.

## Features

- **21 search capabilities** -- Boolean, phrase, proximity, Strong's,
  lemma, morphology, semantic domains, cross-references, syntax/clause,
  discourse analysis, LLM-powered semantic search, concept search,
  query expansion, hybrid ranking, saved searches, and visual query
  builder
- **Full SWORD compatibility** -- reads KJV, ESV, NASB, and thousands of
  modules in RawText, zText, RawCom, zCom, RawLD, zLD, RawGenBook, zGenBook formats
- **Discourse analysis** -- 22 relationship types (Series, Ground,
  Inference, Purpose, etc.) with arcing, bracketing, and phrasing
- **AI integration** -- LLM provider abstraction, embedding-based
  vector search, concept mapping, query expansion
- **Cross-reference graph** -- BFS traversal over Scripture parallels
  with SQLite adjacency list storage
- **Morphology** -- Robinson Greek + OSHM Hebrew codes, 10 facet fields
  in Tantivy, 9 query prefixes
- **Multiple integration surfaces** -- CLI, GUI (Slint), REST API (Axum),
  C FFI, WASM, with a visual query builder

## Quick Start

```bash
# Build the CLI
cargo build --release -p rhema_cli_chirho

# List available SWORD modules
rhema_chirho modules

# Search for a term
rhema_chirho search KJV "love AND world"

# Strong's number lookup
rhema_chirho search KJV "strong:G26"

# Morphology search (Greek aorist verbs)
rhema_chirho search KJV "tense:AoristChirho"

# Cross-reference expansion
rhema_chirho search KJV "xref:John.3.16"

# Parse and explain a query without executing
rhema_chirho parse "love NEAR/5 world [John]"
```

## Crate Map

| Crate | Purpose |
|-------|---------|
| `rhema_contracts_chirho` | Shared types, DTOs, and trait definitions |
| `rhema_core_chirho` | Core engine: corpus management, configuration |
| `rhema_ingest_chirho` | SWORD module reading, token extraction |
| `rhema_index_chirho` | Tantivy full-text indexing (19 fields) |
| `rhema_query_chirho` | Query parser, IR, and planner |
| `rhema_exec_chirho` | Query execution runtime |
| `rhema_module_chirho` | SQLite `.rhema` module format |
| `rhema_ai_chirho` | LLM providers, embeddings, vector store |
| `rhema_discourse_chirho` | Discourse analysis (22 relationship types) |
| `rhema_integration_chirho` | High-level orchestration glue |
| `rhema_cli_chirho` | Command-line interface |
| `rhema_gui_chirho` | GUI components + visual query builder |
| `rhema_api_chirho` | REST API (Axum) + Biblia client + Accordance URLs |
| `rhema_ffi_chirho` | C FFI bindings |
| `rhema_wasm_chirho` | WASM target for browser search |
| `rhema_accel_chirho` | SIMD acceleration (AVX2 bitset ops) |
| `rhema_testkit_chirho` | Test fixtures and golden harnesses |
| `rhema_bench_chirho` | Performance benchmarks |

## Query Syntax

```
love                        # Simple term
"God so loved"              # Exact phrase
love AND world              # Boolean AND
love OR grace               # Boolean OR
love NOT hate               # Inline NOT
love NEAR/5 world           # Proximity (within 5 words)
strong:G26                  # Strong's number
lemma:agape                 # Dictionary form
morph:V-PAI-3S              # Raw morphology code
pos:VerbChirho              # Part of speech
tense:AoristChirho          # Grammatical tense
domain:love                 # Semantic domain
sense:love.01               # Word sense
syntax:relative             # Clause type
xref:John.3.16              # Cross-reference expansion
XREF/2 Gen.1.1              # Multi-hop cross-references
rel:ground [Romans]         # Discourse relationship
prop:"resurrection"         # Proposition text
semantic:redemption         # Vector similarity search
~salvation                  # Shorthand for semantic:
concept:atonement           # Thematic concept mapping
expand:forgiveness          # LLM query expansion
[John]                      # Book scope
[OT]                        # Testament scope
[Genesis-Deuteronomy]       # Range scope
```

## Building

```bash
# Check the workspace
cargo check --workspace

# Run all tests (483 tests)
cargo test --workspace

# Run clippy
cargo clippy --workspace

# Build with the fast release profile
cargo build --profile release-fast-chirho -p rhema_cli_chirho
```

## Architecture

```
                    +-----------------+
                    |   CLI / GUI /   |
                    |  API / FFI /    |
                    |     WASM        |
                    +--------+--------+
                             |
                    +--------v--------+
                    |  Integration    |
                    +--------+--------+
                             |
              +--------------+--------------+
              |              |              |
     +--------v---+  +------v------+  +----v-------+
     |   Query    |  |    Exec     |  |     AI     |
     |  Parser +  |  |  Executor   |  | Embeddings |
     |  Planner   |  |  (Tantivy/  |  | Expansion  |
     |            |  |   Regex)    |  |  Concepts  |
     +-----+------+  +------+------+  +----+-------+
           |                |              |
     +-----v------+  +-----v------+  +----v-------+
     |  Contracts |  |   Index    |  | Discourse  |
     |  (Types,   |  |  (Tantivy  |  | (22 rels,  |
     |   DTOs)    |  |   Schema)  |  |  arcing)   |
     +-----+------+  +-----+------+  +----+-------+
           |                |              |
     +-----v------+  +-----v------+       |
     |   Ingest   |  |   Module   |<------+
     | (SWORD IO) |  |  (.rhema   |
     |            |  |   SQLite)  |
     +-----+------+  +------------+
           |
     +-----v------+
     | rsword_    |
     | chirho     |
     | (SWORD     |
     |  library)  |
     +------------+
```

## License

GPL-2.0-or-later

## Contributing

All identifiers in this project use the **Chirho** suffix convention
(`_chirho` for snake_case, `Chirho` for PascalCase). Every source file
begins with a John 3:16 comment header. Please follow these conventions
in contributions.

> *"The grass withers, the flower fades,
> but the word of our God will stand forever."*
> **-- Isaiah 40:8**
