<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Search Capabilities Matrix (Auto-Generated)

This document tracks rhema_chirho search features vs Logos/Accordance/BibleArc parity.

## Feature Matrix

| Feature | Status | Parser | Planner | Executor | Tests | Notes |
|---------|--------|--------|---------|----------|-------|-------|
| Boolean AND/OR | **implemented** | yes | yes | yes | yes |  |
| NOT Exclusion | **implemented** | yes | yes | yes | yes | Inline NOT in AND context; standalone NOT returns empty |
| Quoted Phrase | **implemented** | yes | yes | yes | yes |  |
| Proximity (NEAR/N) | **implemented** | yes | yes | yes | yes |  |
| Fuzzy Search | **implemented** | - | - | yes | yes | Available via IndexSearcherChirho API, not parser |
| Regex Search | **implemented** | - | - | yes | yes | Available via IndexSearcherChirho API, not parser |
| Strong's Number Lookup | **implemented** | yes | yes | yes | yes |  |
| Lemma Search | **implemented** | yes | yes | yes | yes | Falls back to full-text search on lemma string |
| Scope Filtering (Book/Testament) | **implemented** | yes | yes | yes | yes |  |
| Implicit AND (multi-word) | **implemented** | yes | yes | yes | yes |  |
| Morphological Search | **planned** | - | - | - | - | Types defined in contracts, planner stub exists, needs parser+index+executor |
| Semantic Domain Search | **planned** | - | - | - | - | Requires semantic-chirho SQLite bridge |
| Cross-Reference Graph Search | **planned** | - | - | - | - | GraphExpandChirho node exists in IR, planner stub exists |
| Syntax/Clause Search | **planned** | - | - | - | - | Phase D research track (ETCBC/BHSA data) |
| Discourse Analysis (Arcing) | **implemented** | yes | yes | yes | yes | 22 relationship types, rel: and prop: prefixes, SQLite storage |
| Semantic Search (LLM Embeddings) | **implemented** | yes | yes | yes | yes | semantic: and ~ prefixes, SQLite vector store, cosine similarity |
| Concept Search | **implemented** | yes | yes | yes | yes | concept: prefix, maps natural language to biblical themes via LLM |
| Query Expansion (LLM) | **implemented** | yes | yes | yes | yes | expand: prefix, LLM-powered term/Strong's expansion |
| Hybrid Search (keyword + semantic) | **implemented** | yes | yes | yes | yes | Weighted merge + RRF strategies, configurable keyword weight |
| Saved Searches | **planned** | - | - | - | - |  |
| Visual Query Builder | **planned** | - | - | - | - | Phase D, depends on Slint GUI work |

## Summary

- **Implemented:** 15/21
- **Stubbed:** 0/21
- **Planned:** 6/21

## Logos/Accordance/BibleArc Comparison

| Feature | Logos | Accordance | BibleArc | rhema_chirho |
|---------|-------|------------|----------|-------------|
| Boolean Search | yes | yes | - | **implemented** |
| Phrase Search | yes | yes | - | **implemented** |
| Proximity | yes | yes | - | **implemented** |
| Strong's | yes | yes | - | **implemented** |
| Lemma | yes | yes | - | **implemented** |
| Morphology | yes | yes | - | planned |
| Semantic Domain | yes | yes | - | planned |
| Syntax Search | yes | yes | - | planned (Phase D) |
| Cross-references | yes | yes | - | planned |
| Discourse Analysis | - | - | yes | planned (Phase D) |
| Visual Query Builder | yes | yes | - | planned (Phase D) |
| Saved Searches | yes | yes | - | planned |

## Platform Integrations

| Platform | Method | Status | Feasibility | Notes |
|----------|--------|--------|-------------|-------|
| Logos | Biblia REST API | **planned** | high | Free API for Bible text lookup/search |
| Logos | SWORD Module Compatibility | **implemented** | high | rsword_chirho reads same SWORD modules |
| Accordance | URL Scheme Deep Linking | **planned** | high | Generate accordance.bible/link/ URLs |
| Accordance | Plain Text Import/Export | **implemented** | high | Via mod2imp/imp2vs tools in rsword_chirho |
| BibleArc | No Public API | **blocked** | none | Web-only, no export, no developer docs |
| semantic-chirho | SQLite Bridge Import | **planned** | high | Token→sense→domain pipeline with 38+ language translation signatures |

## Open Data Sources for Advanced Search

| Dataset | License | Purpose | Status |
|---------|---------|---------|--------|
| Robinson RMAC | CC-BY-SA 3.0 | Greek morphology codes | Available via SWORD |
| OSHM codes | In SWORD modules | Hebrew morphology | Available via SWORD |
| MorphGNT/SBLGNT | CC-BY-SA | Full Greek morphological tagging | Not yet imported |
| OSHB/MorphHB | CC BY 4.0 | Complete Hebrew morphology | Not yet imported |
| STEP Bible TAGNT/TAHOT | CC BY 4.0 | Expanded morph codes | Not yet imported |
| SDGNT (Louw-Nida) | CC-BY-SA 4.0 | Semantic domains for Greek | Not yet imported |
| UBS Parallel Passages | CC-BY-SA 4.0 | Cross-reference data | Not yet imported |
| TSK | Public domain | Cross-references (via SWORD) | Not yet imported |
| semantic-chirho | Internal | Token-sense-domain pipeline | Planned bridge |
| ETCBC/BHSA | Research use | Hebrew syntax trees | Phase D |
| OpenText.org | Research | Greek linguistic annotations | Phase D |

---
*Generated by gen_docs_chirho.ts on 2026-02-08*