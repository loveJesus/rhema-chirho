<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# rhema_chirho Architecture (Auto-Generated)

> Biblical scholarship engine in Pure Rust



## Layer Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    Integration Surfaces                      │
│  rhema_cli  rhema_gui  rhema_api  rhema_ai  rhema_ffi/wasm │
├──────────────────────────────────────────────────────────────┤
│                     rhema_integration                        │
├──────────────────────────────────────────────────────────────┤
│                      Core Engine                             │
│  rhema_exec  ←  rhema_query  ←  rhema_index  ←  rhema_core │
│                                     ↑                       │
│                              rhema_ingest                    │
├──────────────────────────────────────────────────────────────┤
│                    rhema_contracts (types + traits)          │
├──────────────────────────────────────────────────────────────┤
│              rsword_chirho (SWORD binary compat)             │
└──────────────────────────────────────────────────────────────┘
```

## Data Flow

```
User Query → parser_chirho → QueryNodeChirho (IR) → planner_chirho → PlanStepChirho → executor_chirho → QueryResultChirho
              ↓                                        ↓
         strong:, lemma:,                         Tantivy index
         NEAR/N, AND/OR/NOT                       or regex fallback
```

## Crate Details

### Core Layer

#### `rhema_contracts_chirho`

Shared type-level contracts, newtypes, DTOs, and trait definitions

**Modules:** ids_chirho, keys_chirho, query_chirho, corpus_chirho, morphology_chirho, morph_parser_chirho, result_chirho, error_chirho, capability_chirho

**Capabilities:**

- [x] Verse Reference Types
- [x] Query IR (QueryNodeChirho)
- [x] Morphology Types
- [x] Result DTOs
- [x] Capability Traits
- [x] Strong's Number Types

#### `rhema_core_chirho`

Core engine logic: corpus management, module resolution, and configuration

**Capabilities:**

- [x] Core Engine

#### `rhema_ingest_chirho`

Ingestion adapters: SWORD module reading, token extraction, chapter/verse iteration

**Modules:** sword_adapter_chirho, token_extractor_chirho

**Capabilities:**

- [x] SWORD Module Reading
- [x] Token Extraction (Strong's, Lemma, Morph)

#### `rhema_index_chirho`

Full-text indexing engine: Tantivy schema, indexer, searcher, manifest versioning

**Modules:** schema_chirho, indexer_chirho, searcher_chirho, manifest_chirho, error_chirho

**Capabilities:**

- [x] Tantivy Schema (key, book, chapter, verse, text, strongs, module)
- [x] Module Indexer (build full-text index from SWORD module)
- [x] Full-text Search (boolean, phrase, fuzzy, regex)
- [x] Strong's Number Search
- [x] Book-scoped Search
- [x] Manifest Versioning
- [x] Morphology Index Fields (morph, lemma, pos, tense, voice, mood, case, number, gender, person)
- [ ] Semantic Domain Index Fields (sense, domain)
- [ ] Cross-Reference Graph Index

#### `rhema_query_chirho`

Query parser, IR, and planner: text queries → QueryNodeChirho → PlanStepChirho

**Modules:** parser_chirho, planner_chirho, error_chirho

**Capabilities:**

- [x] Simple Term Parsing
- [x] Quoted Phrase Parsing
- [x] Boolean Operators (AND, OR, NOT)
- [x] Strong's Number Prefix (strong:G26)
- [x] Lemma Prefix (lemma:agape)
- [x] Proximity (NEAR/N)
- [x] Scope Filtering ([Book], [OT], [NT])
- [x] Implicit AND (multi-word)
- [x] Morphology Query Syntax (morph:, pos:, tense:, voice:, mood:, case:, number:, gender:, person:)
- [ ] Semantic Domain Syntax (domain:, sense:)
- [ ] Cross-Reference Syntax (xref:, XREF/N)
- [x] Query Plan Optimization
- [x] Morph Plan Step
- [-] GraphExpand Plan Step

#### `rhema_exec_chirho`

Query execution runtime: dispatches planned queries against Tantivy and regex backends

**Modules:** executor_chirho, error_chirho

**Capabilities:**

- [x] Tantivy-backed Full-text Execution
- [x] Regex Fallback Execution
- [x] Strong's Execution (indexed + fallback)
- [x] AND Intersection Execution
- [x] OR Union Execution
- [x] NOT Exclusion Execution
- [x] Proximity Execution
- [x] Morphology Execution
- [ ] Semantic Domain Execution
- [ ] Cross-Reference Graph Execution
- [ ] Saved Searches Persistence

#### `rhema_module_chirho`

SQLite-based module format: self-contained .rhema files with verses, tokens, morphology, and metadata

**Modules:** schema_chirho, writer_chirho, reader_chirho, morph_query_chirho, converter_chirho, error_chirho

**Capabilities:**

- [x] SQLite Module Schema (verses, tokens, metadata)
- [x] Module Writer (build .rhema from ingest data)
- [x] Module Reader (query verses, tokens, morphology)
- [x] Morphology Query via SQL WHERE
- [x] SWORD → .rhema Converter

### Integration Layer

#### `rhema_integration_chirho`

Glue crate wiring core into higher-level orchestration

**Capabilities:**

- [x] Integration Layer

#### `rhema_cli_chirho`

CLI binary: search, index, and manage modules from the terminal

**Capabilities:**

- [x] CLI Search Command

#### `rhema_gui_chirho`

GUI integration: Slint UI bindings for Codex Lux Chirho

**Capabilities:**

- [x] GUI Bindings

#### `rhema_api_chirho`

REST API server and external platform integration clients

**Capabilities:**

- [ ] REST API
- [ ] Biblia API Client (Logos)
- [ ] Accordance URL Scheme Generator

#### `rhema_ai_chirho`

AI/LLM integration: embedding generation, semantic search, vector store, query expansion, hybrid ranking

**Modules:** llm_provider_chirho, embedding_chirho, vector_store_chirho, query_expander_chirho, hybrid_ranker_chirho, concept_mapper_chirho, indexing_chirho, config_chirho, error_chirho

**Capabilities:**

- [x] LLM Provider Abstraction (OpenAI/Anthropic/local)
- [x] Embedding Model Trait + Mock
- [x] SQLite Vector Store (portable, WASM-compatible)
- [x] LLM Query Expansion (terms, Strong's, concepts)
- [x] Hybrid Ranking (Weighted + RRF)
- [x] Concept Mapping (concept → biblical themes)
- [x] Batch Embedding Indexing Pipeline

#### `rhema_discourse_chirho`

Discourse analysis engine: arcing, bracketing, phrasing with 22 relationship types

**Modules:** types_chirho, storage_chirho, sqlite_store_chirho, detector_chirho, suggester_chirho, validator_chirho, llm_generator_chirho, search_chirho, import_export_chirho, error_chirho

**Capabilities:**

- [x] Core Data Model (22 Relationship Types)
- [x] SQLite Discourse Storage
- [x] Heuristic Proposition Detection
- [x] Rule-Based Relationship Suggestion
- [x] Arc Structure Validation
- [x] LLM-Powered Arc Generation
- [x] Discourse Search Integration
- [x] JSON Import/Export (round-trip)

#### `rhema_ffi_chirho`

C FFI bindings for embedding rhema in C/C++/Swift/Kotlin

**Capabilities:**

- [ ] C FFI Bindings

#### `rhema_wasm_chirho`

WASM build target for browser-based search

**Capabilities:**

- [ ] WASM Target

### Quality Layer

#### `rhema_testkit_chirho`

Shared test fixtures, builders, and assertion helpers

**Capabilities:**

- [x] Test Fixtures

#### `rhema_bench_chirho`

Performance benchmarks for indexing and search

**Capabilities:**

- [x] Benchmarks

### Research Layer

#### `rhema_accel_chirho`

Hardware acceleration research: SIMD, GPU compute via wgpu

**Capabilities:**

- [ ] SIMD Acceleration

---
*Generated by gen_docs_chirho.ts on 2026-02-08*