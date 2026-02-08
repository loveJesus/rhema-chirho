<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# rhema_chirho — AI Context (Auto-Generated)

> Biblical scholarship engine in Pure Rust

Version: 0.1.0 | License: GPL-2.0-or-later

## Crate Map

| Crate | Purpose | Layer | Phase |
|-------|---------|-------|-------|
| `rhema_contracts_chirho` | Shared type-level contracts, newtypes, DTOs, and trait definitions | core | A |
| `rhema_core_chirho` | Core engine logic: corpus management, module resolution, and configuration | core | A |
| `rhema_ingest_chirho` | Ingestion adapters: SWORD module reading, token extraction, chapter/verse iteration | core | A |
| `rhema_index_chirho` | Full-text indexing engine: Tantivy schema, indexer, searcher, manifest versioning | core | A |
| `rhema_query_chirho` | Query parser, IR, and planner: text queries → QueryNodeChirho → PlanStepChirho | core | A |
| `rhema_exec_chirho` | Query execution runtime: dispatches planned queries against Tantivy and regex backends | core | A |
| `rhema_integration_chirho` | Glue crate wiring core into higher-level orchestration | integration | B |
| `rhema_cli_chirho` | CLI binary: search, index, and manage modules from the terminal | integration | B |
| `rhema_gui_chirho` | GUI integration: Slint UI bindings for Codex Lux Chirho | integration | B |
| `rhema_api_chirho` | REST API server and external platform integration clients | integration | B |
| `rhema_ai_chirho` | AI/LLM integration: embedding generation, semantic search, vector store, query expansion, hybrid ranking | integration | B |
| `rhema_discourse_chirho` | Discourse analysis engine: arcing, bracketing, phrasing with 22 relationship types | integration | B |
| `rhema_module_chirho` | SQLite-based module format: self-contained .rhema files with verses, tokens, morphology, and metadata | core | A |
| `rhema_ffi_chirho` | C FFI bindings for embedding rhema in C/C++/Swift/Kotlin | integration | B |
| `rhema_wasm_chirho` | WASM build target for browser-based search | integration | B |
| `rhema_accel_chirho` | Hardware acceleration research: SIMD, GPU compute via wgpu | research | D |
| `rhema_testkit_chirho` | Shared test fixtures, builders, and assertion helpers | quality | A |
| `rhema_bench_chirho` | Performance benchmarks for indexing and search | quality | A |

## Key Types & Traits

### rhema_contracts_chirho
- Traits: `QueryExecutorChirho`, `CorpusReaderChirho`, `IngestAdapterChirho`
- Modules: ids_chirho, keys_chirho, query_chirho, corpus_chirho, morphology_chirho, morph_parser_chirho, result_chirho, error_chirho, capability_chirho

### rhema_ingest_chirho
- Types: `SwordAdapterChirho`, `TokenExtractorChirho`
- Modules: sword_adapter_chirho, token_extractor_chirho

### rhema_index_chirho
- Types: `RhemaSchemaChirho`, `ModuleIndexerChirho`, `IndexSearcherChirho`, `IndexHitChirho`, `IndexManifestChirho`
- Modules: schema_chirho, indexer_chirho, searcher_chirho, manifest_chirho, error_chirho

### rhema_query_chirho
- Types: `QueryParserChirho`, `QueryPlannerChirho`, `PlanStepChirho`, `QueryPlanChirho`
- Modules: parser_chirho, planner_chirho, error_chirho

### rhema_exec_chirho
- Types: `QueryExecutorImplChirho`, `ExecErrorChirho`
- Modules: executor_chirho, error_chirho

## Capability Matrix

| Crate | Capability | Status | Tests |
|-------|-----------|--------|-------|
| rhema_contracts_chirho | Verse Reference Types | DONE | yes |
| rhema_contracts_chirho | Query IR (QueryNodeChirho) | DONE | yes |
| rhema_contracts_chirho | Morphology Types | DONE | yes |
| rhema_contracts_chirho | Result DTOs | DONE | yes |
| rhema_contracts_chirho | Capability Traits | DONE | yes |
| rhema_contracts_chirho | Strong's Number Types | DONE | yes |
| rhema_core_chirho | Core Engine | DONE | yes |
| rhema_ingest_chirho | SWORD Module Reading | DONE | yes |
| rhema_ingest_chirho | Token Extraction (Strong's, Lemma, Morph) | DONE | yes |
| rhema_index_chirho | Tantivy Schema (key, book, chapter, verse, text, strongs, module) | DONE | yes |
| rhema_index_chirho | Module Indexer (build full-text index from SWORD module) | DONE | yes |
| rhema_index_chirho | Full-text Search (boolean, phrase, fuzzy, regex) | DONE | yes |
| rhema_index_chirho | Strong's Number Search | DONE | yes |
| rhema_index_chirho | Book-scoped Search | DONE | yes |
| rhema_index_chirho | Manifest Versioning | DONE | yes |
| rhema_index_chirho | Morphology Index Fields (morph, lemma, pos, tense, voice, mood, case, number, gender, person) | DONE | yes |
| rhema_index_chirho | Semantic Domain Index Fields (sense, domain) | TODO | no |
| rhema_index_chirho | Cross-Reference Graph Index | TODO | no |
| rhema_query_chirho | Simple Term Parsing | DONE | yes |
| rhema_query_chirho | Quoted Phrase Parsing | DONE | yes |
| rhema_query_chirho | Boolean Operators (AND, OR, NOT) | DONE | yes |
| rhema_query_chirho | Strong's Number Prefix (strong:G26) | DONE | yes |
| rhema_query_chirho | Lemma Prefix (lemma:agape) | DONE | yes |
| rhema_query_chirho | Proximity (NEAR/N) | DONE | yes |
| rhema_query_chirho | Scope Filtering ([Book], [OT], [NT]) | DONE | yes |
| rhema_query_chirho | Implicit AND (multi-word) | DONE | yes |
| rhema_query_chirho | Morphology Query Syntax (morph:, pos:, tense:, voice:, mood:, case:, number:, gender:, person:) | DONE | yes |
| rhema_query_chirho | Semantic Domain Syntax (domain:, sense:) | TODO | no |
| rhema_query_chirho | Cross-Reference Syntax (xref:, XREF/N) | TODO | no |
| rhema_query_chirho | Query Plan Optimization | DONE | yes |
| rhema_query_chirho | Morph Plan Step | DONE | yes |
| rhema_query_chirho | GraphExpand Plan Step | STUB | no |
| rhema_exec_chirho | Tantivy-backed Full-text Execution | DONE | yes |
| rhema_exec_chirho | Regex Fallback Execution | DONE | yes |
| rhema_exec_chirho | Strong's Execution (indexed + fallback) | DONE | yes |
| rhema_exec_chirho | AND Intersection Execution | DONE | yes |
| rhema_exec_chirho | OR Union Execution | DONE | yes |
| rhema_exec_chirho | NOT Exclusion Execution | DONE | yes |
| rhema_exec_chirho | Proximity Execution | DONE | yes |
| rhema_exec_chirho | Morphology Execution | DONE | yes |
| rhema_exec_chirho | Semantic Domain Execution | TODO | no |
| rhema_exec_chirho | Cross-Reference Graph Execution | TODO | no |
| rhema_exec_chirho | Saved Searches Persistence | TODO | no |
| rhema_integration_chirho | Integration Layer | DONE | yes |
| rhema_cli_chirho | CLI Search Command | DONE | yes |
| rhema_gui_chirho | GUI Bindings | DONE | no |
| rhema_api_chirho | REST API | TODO | no |
| rhema_api_chirho | Biblia API Client (Logos) | TODO | no |
| rhema_api_chirho | Accordance URL Scheme Generator | TODO | no |
| rhema_ai_chirho | LLM Provider Abstraction (OpenAI/Anthropic/local) | DONE | yes |
| rhema_ai_chirho | Embedding Model Trait + Mock | DONE | yes |
| rhema_ai_chirho | SQLite Vector Store (portable, WASM-compatible) | DONE | yes |
| rhema_ai_chirho | LLM Query Expansion (terms, Strong's, concepts) | DONE | yes |
| rhema_ai_chirho | Hybrid Ranking (Weighted + RRF) | DONE | yes |
| rhema_ai_chirho | Concept Mapping (concept → biblical themes) | DONE | yes |
| rhema_ai_chirho | Batch Embedding Indexing Pipeline | DONE | yes |
| rhema_discourse_chirho | Core Data Model (22 Relationship Types) | DONE | yes |
| rhema_discourse_chirho | SQLite Discourse Storage | DONE | yes |
| rhema_discourse_chirho | Heuristic Proposition Detection | DONE | yes |
| rhema_discourse_chirho | Rule-Based Relationship Suggestion | DONE | yes |
| rhema_discourse_chirho | Arc Structure Validation | DONE | yes |
| rhema_discourse_chirho | LLM-Powered Arc Generation | DONE | yes |
| rhema_discourse_chirho | Discourse Search Integration | DONE | yes |
| rhema_discourse_chirho | JSON Import/Export (round-trip) | DONE | yes |
| rhema_module_chirho | SQLite Module Schema (verses, tokens, metadata) | DONE | yes |
| rhema_module_chirho | Module Writer (build .rhema from ingest data) | DONE | yes |
| rhema_module_chirho | Module Reader (query verses, tokens, morphology) | DONE | yes |
| rhema_module_chirho | Morphology Query via SQL WHERE | DONE | yes |
| rhema_module_chirho | SWORD → .rhema Converter | DONE | yes |
| rhema_ffi_chirho | C FFI Bindings | TODO | no |
| rhema_wasm_chirho | WASM Target | TODO | no |
| rhema_accel_chirho | SIMD Acceleration | TODO | no |
| rhema_testkit_chirho | Test Fixtures | DONE | yes |
| rhema_bench_chirho | Benchmarks | DONE | no |

## Search Feature Status

| Feature | Status | Parser | Planner | Executor | Tests |
|---------|--------|--------|---------|----------|-------|
| Boolean AND/OR | implemented | yes | yes | yes | yes |
| NOT Exclusion | implemented | yes | yes | yes | yes |
| Quoted Phrase | implemented | yes | yes | yes | yes |
| Proximity (NEAR/N) | implemented | yes | yes | yes | yes |
| Fuzzy Search | implemented | no | no | yes | yes |
| Regex Search | implemented | no | no | yes | yes |
| Strong's Number Lookup | implemented | yes | yes | yes | yes |
| Lemma Search | implemented | yes | yes | yes | yes |
| Scope Filtering (Book/Testament) | implemented | yes | yes | yes | yes |
| Implicit AND (multi-word) | implemented | yes | yes | yes | yes |
| Morphological Search | implemented | yes | yes | yes | yes |
| Semantic Domain Search | planned | no | no | no | no |
| Cross-Reference Graph Search | planned | no | no | no | no |
| Syntax/Clause Search | planned | no | no | no | no |
| Discourse Analysis (Arcing) | implemented | yes | yes | yes | yes |
| Semantic Search (LLM Embeddings) | implemented | yes | yes | yes | yes |
| Concept Search | implemented | yes | yes | yes | yes |
| Query Expansion (LLM) | implemented | yes | yes | yes | yes |
| Hybrid Search (keyword + semantic) | implemented | yes | yes | yes | yes |
| Saved Searches | planned | no | no | no | no |
| Visual Query Builder | planned | no | no | no | no |

## Dependency Graph (Adjacency List)

- `rhema_contracts_chirho` → (none)
- `rhema_core_chirho` → rhema_contracts_chirho
- `rhema_ingest_chirho` → rhema_contracts_chirho, rsword_chirho
- `rhema_index_chirho` → rhema_contracts_chirho, rhema_ingest_chirho, rsword_chirho
- `rhema_query_chirho` → rhema_contracts_chirho, rhema_index_chirho
- `rhema_exec_chirho` → rhema_contracts_chirho, rhema_query_chirho, rhema_ingest_chirho, rhema_index_chirho, rhema_ai_chirho, rsword_chirho
- `rhema_integration_chirho` → rhema_contracts_chirho, rhema_core_chirho, rhema_exec_chirho
- `rhema_cli_chirho` → rhema_contracts_chirho, rhema_exec_chirho
- `rhema_gui_chirho` → rhema_contracts_chirho
- `rhema_api_chirho` → rhema_contracts_chirho
- `rhema_ai_chirho` → rhema_contracts_chirho, async-trait, rusqlite
- `rhema_discourse_chirho` → rhema_contracts_chirho, rhema_ai_chirho, rusqlite
- `rhema_module_chirho` → rhema_contracts_chirho, rhema_ingest_chirho, rusqlite
- `rhema_ffi_chirho` → rhema_contracts_chirho
- `rhema_wasm_chirho` → rhema_contracts_chirho
- `rhema_accel_chirho` → rhema_contracts_chirho
- `rhema_testkit_chirho` → rhema_contracts_chirho
- `rhema_bench_chirho` → rhema_contracts_chirho, rhema_exec_chirho

## Platform Integrations

| Platform | Method | Status | Feasibility |
|----------|--------|--------|-------------|
| Logos | Biblia REST API | planned | high |
| Logos | SWORD Module Compatibility | implemented | high |
| Accordance | URL Scheme Deep Linking | planned | high |
| Accordance | Plain Text Import/Export | implemented | high |
| BibleArc | No Public API | blocked | none |
| semantic-chirho | SQLite Bridge Import | planned | high |

## Key File Paths

```
rhema_chirho/
  Cargo.toml                    # Workspace manifest
  docs_chirho/
    manifest_chirho.toml        # Master doc manifest (source of truth)
    AI_CONTEXT_CHIRHO.md        # This file (auto-generated)
    ARCHITECTURE_CHIRHO.md      # Human architecture doc
    SEARCH_CAPABILITIES_CHIRHO.md # Search feature matrix
  crates/
    rhema_contracts_chirho/
    rhema_core_chirho/
    rhema_ingest_chirho/
    rhema_index_chirho/
    rhema_query_chirho/
    rhema_exec_chirho/
    rhema_integration_chirho/
    rhema_cli_chirho/
    rhema_gui_chirho/
    rhema_api_chirho/
    rhema_ai_chirho/
    rhema_discourse_chirho/
    rhema_module_chirho/
    rhema_ffi_chirho/
    rhema_wasm_chirho/
    rhema_accel_chirho/
    rhema_testkit_chirho/
    rhema_bench_chirho/
```

---
*Generated by gen_docs_chirho.ts on 2026-02-08*