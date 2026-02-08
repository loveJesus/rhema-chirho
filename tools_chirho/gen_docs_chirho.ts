// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

/**
 * gen_docs_chirho.ts — Generates documentation from manifest_chirho.toml
 *
 * Reads the master manifest and produces:
 *   1. AI_CONTEXT_CHIRHO.md   — LLM-optimized context file (<500 lines)
 *   2. ARCHITECTURE_CHIRHO.md — Human-readable architecture overview
 *   3. SEARCH_CAPABILITIES_CHIRHO.md — Search feature matrix
 *
 * Usage: bun run rhema_chirho/tools_chirho/gen_docs_chirho.ts
 */

import { readFileSync, writeFileSync, mkdirSync } from "fs";
import { join, dirname } from "path";

// ─── TOML Parser (minimal, no external deps) ──────────────────

interface CrateCapabilityChirho {
  name_chirho: string;
  status_chirho: string;
  tests_chirho: boolean;
}

interface CrateEntryChirho {
  name_chirho: string;
  purpose_chirho: string;
  layer_chirho: string;
  phase_chirho: string;
  modules_chirho: string[];
  key_types_chirho: string[];
  key_traits_chirho?: string[];
  dependencies_chirho: string[];
  capability_chirho: CrateCapabilityChirho[];
}

interface SearchFeatureChirho {
  name_chirho: string;
  status_chirho: string;
  parser_chirho: boolean;
  planner_chirho: boolean;
  executor_chirho: boolean;
  tests_chirho: boolean;
  note_chirho?: string;
}

interface IntegrationEntryChirho {
  platform_chirho: string;
  method_chirho: string;
  status_chirho: string;
  feasibility_chirho: string;
  note_chirho?: string;
}

interface ProjectInfoChirho {
  name_chirho: string;
  tagline_chirho: string;
  version_chirho: string;
  license_chirho: string;
  repository_chirho: string;
  description_chirho: string;
}

interface ManifestChirho {
  project_chirho: ProjectInfoChirho;
  crate_chirho: CrateEntryChirho[];
  search_feature_chirho: SearchFeatureChirho[];
  integration_chirho: IntegrationEntryChirho[];
  dependency_graph_chirho: Record<string, string[]>;
}

/**
 * Minimal TOML parser for our manifest format.
 * Handles tables, arrays of tables, strings, booleans, and string arrays.
 */
function parseTomlChirho(contentChirho: string): ManifestChirho {
  const linesChirho = contentChirho.split("\n");
  const resultChirho: any = {};

  let currentTableChirho: string | null = null;
  let currentArrayTableChirho: string | null = null;
  let currentObjChirho: any = null;
  // Track sub-array tables like [[crate_chirho.capability_chirho]]
  let parentArrayTableChirho: string | null = null;
  let parentObjChirho: any = null;

  for (const rawLineChirho of linesChirho) {
    const lineChirho = rawLineChirho.trim();

    // Skip comments and empty lines
    if (lineChirho === "" || lineChirho.startsWith("#")) continue;

    // Multi-line string continuation (backslash)
    // handled by joining at assignment

    // Array of tables: [[table_name]]
    const arrayTableMatchChirho = lineChirho.match(
      /^\[\[([a-zA-Z0-9_.-]+)\]\]$/
    );
    if (arrayTableMatchChirho) {
      const fullNameChirho = arrayTableMatchChirho[1];

      // Check if it's a sub-table like crate_chirho.capability_chirho
      if (fullNameChirho.includes(".")) {
        const partsChirho = fullNameChirho.split(".");
        const parentKeyChirho = partsChirho[0];
        const childKeyChirho = partsChirho[1];

        // The parent should be the most recent entry in the parent array
        if (resultChirho[parentKeyChirho]) {
          parentObjChirho =
            resultChirho[parentKeyChirho][
              resultChirho[parentKeyChirho].length - 1
            ];
          if (!parentObjChirho[childKeyChirho]) {
            parentObjChirho[childKeyChirho] = [];
          }
          currentObjChirho = {};
          parentObjChirho[childKeyChirho].push(currentObjChirho);
          parentArrayTableChirho = childKeyChirho;
        }
        currentTableChirho = null;
        currentArrayTableChirho = null;
      } else {
        if (!resultChirho[fullNameChirho]) {
          resultChirho[fullNameChirho] = [];
        }
        currentObjChirho = {};
        resultChirho[fullNameChirho].push(currentObjChirho);
        currentArrayTableChirho = fullNameChirho;
        currentTableChirho = null;
        parentArrayTableChirho = null;
      }
      continue;
    }

    // Regular table: [table_name]
    const tableMatchChirho = lineChirho.match(/^\[([a-zA-Z0-9_.-]+)\]$/);
    if (tableMatchChirho) {
      const tableNameChirho = tableMatchChirho[1];
      if (!resultChirho[tableNameChirho]) {
        resultChirho[tableNameChirho] = {};
      }
      currentObjChirho = resultChirho[tableNameChirho];
      currentTableChirho = tableNameChirho;
      currentArrayTableChirho = null;
      parentArrayTableChirho = null;
      continue;
    }

    // Key-value pair
    const kvMatchChirho = lineChirho.match(/^([a-zA-Z0-9_]+)\s*=\s*(.+)$/);
    if (kvMatchChirho && currentObjChirho) {
      const keyChirho = kvMatchChirho[1];
      let valueStrChirho = kvMatchChirho[2].trim();

      // Parse value
      const parsedValueChirho = parseTomlValueChirho(valueStrChirho);
      currentObjChirho[keyChirho] = parsedValueChirho;
    }
  }

  return resultChirho as ManifestChirho;
}

function parseTomlValueChirho(valueChirho: string): any {
  // Boolean
  if (valueChirho === "true") return true;
  if (valueChirho === "false") return false;

  // Number
  if (/^\d+(\.\d+)?$/.test(valueChirho)) return Number(valueChirho);

  // Multi-line string (triple quotes)
  if (valueChirho.startsWith('"""')) {
    return valueChirho.replace(/^"""|"""$/g, "").replace(/\\\n\s*/g, "").trim();
  }

  // Regular string
  if (valueChirho.startsWith('"') && valueChirho.endsWith('"')) {
    return valueChirho.slice(1, -1);
  }

  // Array of strings
  if (valueChirho.startsWith("[")) {
    const innerChirho = valueChirho.slice(1, -1).trim();
    if (innerChirho === "") return [];
    return innerChirho.split(",").map((itemChirho: string) => {
      const trimmedChirho = itemChirho.trim();
      if (trimmedChirho.startsWith('"') && trimmedChirho.endsWith('"')) {
        return trimmedChirho.slice(1, -1);
      }
      return trimmedChirho;
    });
  }

  return valueChirho;
}

// ─── Document Generators ──────────────────────────────────────

function generateAiContextChirho(manifestChirho: ManifestChirho): string {
  const linesChirho: string[] = [];
  const projectChirho = manifestChirho.project_chirho;

  linesChirho.push(
    "<!-- For God so loved the world that he gave his only begotten Son,"
  );
  linesChirho.push(
    "     that whoever believes in him should not perish but have eternal life."
  );
  linesChirho.push("     John 3:16 -->");
  linesChirho.push("");
  linesChirho.push(
    `# ${projectChirho.name_chirho} — AI Context (Auto-Generated)`
  );
  linesChirho.push("");
  linesChirho.push(`> ${projectChirho.tagline_chirho}`);
  linesChirho.push("");
  linesChirho.push(
    `Version: ${projectChirho.version_chirho} | License: ${projectChirho.license_chirho}`
  );
  linesChirho.push("");

  // Crate summary table
  linesChirho.push("## Crate Map");
  linesChirho.push("");
  linesChirho.push("| Crate | Purpose | Layer | Phase |");
  linesChirho.push("|-------|---------|-------|-------|");

  const cratesChirho = manifestChirho.crate_chirho || [];
  for (const crateChirho of cratesChirho) {
    linesChirho.push(
      `| \`${crateChirho.name_chirho}\` | ${crateChirho.purpose_chirho} | ${crateChirho.layer_chirho} | ${crateChirho.phase_chirho} |`
    );
  }
  linesChirho.push("");

  // Key types per crate
  linesChirho.push("## Key Types & Traits");
  linesChirho.push("");
  for (const crateChirho of cratesChirho) {
    if (
      crateChirho.key_types_chirho?.length > 0 ||
      crateChirho.key_traits_chirho?.length
    ) {
      linesChirho.push(`### ${crateChirho.name_chirho}`);
      if (crateChirho.key_types_chirho?.length > 0) {
        linesChirho.push(
          `- Types: ${crateChirho.key_types_chirho.map((tChirho: string) => `\`${tChirho}\``).join(", ")}`
        );
      }
      if (crateChirho.key_traits_chirho?.length) {
        linesChirho.push(
          `- Traits: ${crateChirho.key_traits_chirho.map((tChirho: string) => `\`${tChirho}\``).join(", ")}`
        );
      }
      if (crateChirho.modules_chirho?.length > 0) {
        linesChirho.push(
          `- Modules: ${crateChirho.modules_chirho.join(", ")}`
        );
      }
      linesChirho.push("");
    }
  }

  // Capability matrix
  linesChirho.push("## Capability Matrix");
  linesChirho.push("");
  linesChirho.push("| Crate | Capability | Status | Tests |");
  linesChirho.push("|-------|-----------|--------|-------|");

  for (const crateChirho of cratesChirho) {
    const capsChirho = crateChirho.capability_chirho || [];
    for (const capChirho of capsChirho) {
      const statusEmojiChirho =
        capChirho.status_chirho === "implemented"
          ? "DONE"
          : capChirho.status_chirho === "stubbed"
            ? "STUB"
            : "TODO";
      const testsEmojiChirho = capChirho.tests_chirho ? "yes" : "no";
      linesChirho.push(
        `| ${crateChirho.name_chirho} | ${capChirho.name_chirho} | ${statusEmojiChirho} | ${testsEmojiChirho} |`
      );
    }
  }
  linesChirho.push("");

  // Search features
  linesChirho.push("## Search Feature Status");
  linesChirho.push("");
  linesChirho.push(
    "| Feature | Status | Parser | Planner | Executor | Tests |"
  );
  linesChirho.push(
    "|---------|--------|--------|---------|----------|-------|"
  );

  const featuresChirho = manifestChirho.search_feature_chirho || [];
  for (const featChirho of featuresChirho) {
    const yesNoChirho = (boolChirho: boolean) => (boolChirho ? "yes" : "no");
    linesChirho.push(
      `| ${featChirho.name_chirho} | ${featChirho.status_chirho} | ${yesNoChirho(featChirho.parser_chirho)} | ${yesNoChirho(featChirho.planner_chirho)} | ${yesNoChirho(featChirho.executor_chirho)} | ${yesNoChirho(featChirho.tests_chirho)} |`
    );
  }
  linesChirho.push("");

  // Dependency graph
  linesChirho.push("## Dependency Graph (Adjacency List)");
  linesChirho.push("");
  const graphChirho = manifestChirho.dependency_graph_chirho || {};
  for (const [crateNameChirho, depsChirho] of Object.entries(graphChirho)) {
    if (Array.isArray(depsChirho) && depsChirho.length > 0) {
      linesChirho.push(`- \`${crateNameChirho}\` → ${depsChirho.join(", ")}`);
    } else {
      linesChirho.push(`- \`${crateNameChirho}\` → (none)`);
    }
  }
  linesChirho.push("");

  // Integrations
  linesChirho.push("## Platform Integrations");
  linesChirho.push("");
  linesChirho.push("| Platform | Method | Status | Feasibility |");
  linesChirho.push("|----------|--------|--------|-------------|");

  const integrationsChirho = manifestChirho.integration_chirho || [];
  for (const intChirho of integrationsChirho) {
    linesChirho.push(
      `| ${intChirho.platform_chirho} | ${intChirho.method_chirho} | ${intChirho.status_chirho} | ${intChirho.feasibility_chirho} |`
    );
  }
  linesChirho.push("");

  // File paths
  linesChirho.push("## Key File Paths");
  linesChirho.push("");
  linesChirho.push("```");
  linesChirho.push("rhema_chirho/");
  linesChirho.push("  Cargo.toml                    # Workspace manifest");
  linesChirho.push("  docs_chirho/");
  linesChirho.push(
    "    manifest_chirho.toml        # Master doc manifest (source of truth)"
  );
  linesChirho.push(
    "    AI_CONTEXT_CHIRHO.md        # This file (auto-generated)"
  );
  linesChirho.push("    ARCHITECTURE_CHIRHO.md      # Human architecture doc");
  linesChirho.push(
    "    SEARCH_CAPABILITIES_CHIRHO.md # Search feature matrix"
  );
  linesChirho.push("  crates/");
  for (const crateChirho of cratesChirho) {
    linesChirho.push(`    ${crateChirho.name_chirho}/`);
  }
  linesChirho.push("```");
  linesChirho.push("");
  linesChirho.push(
    "---"
  );
  linesChirho.push(
    `*Generated by gen_docs_chirho.ts on ${new Date().toISOString().split("T")[0]}*`
  );

  return linesChirho.join("\n");
}

function generateArchitectureChirho(manifestChirho: ManifestChirho): string {
  const linesChirho: string[] = [];
  const projectChirho = manifestChirho.project_chirho;

  linesChirho.push(
    "<!-- For God so loved the world that he gave his only begotten Son,"
  );
  linesChirho.push(
    "     that whoever believes in him should not perish but have eternal life."
  );
  linesChirho.push("     John 3:16 -->");
  linesChirho.push("");
  linesChirho.push(
    `# ${projectChirho.name_chirho} Architecture (Auto-Generated)`
  );
  linesChirho.push("");
  linesChirho.push(`> ${projectChirho.tagline_chirho}`);
  linesChirho.push("");
  linesChirho.push(projectChirho.description_chirho || "");
  linesChirho.push("");

  // Layer diagram
  linesChirho.push("## Layer Architecture");
  linesChirho.push("");
  linesChirho.push("```");
  linesChirho.push(
    "┌──────────────────────────────────────────────────────────────┐"
  );
  linesChirho.push(
    "│                    Integration Surfaces                      │"
  );
  linesChirho.push(
    "│  rhema_cli  rhema_gui  rhema_api  rhema_ai  rhema_ffi/wasm │"
  );
  linesChirho.push(
    "├──────────────────────────────────────────────────────────────┤"
  );
  linesChirho.push(
    "│                     rhema_integration                        │"
  );
  linesChirho.push(
    "├──────────────────────────────────────────────────────────────┤"
  );
  linesChirho.push(
    "│                      Core Engine                             │"
  );
  linesChirho.push(
    "│  rhema_exec  ←  rhema_query  ←  rhema_index  ←  rhema_core │"
  );
  linesChirho.push(
    "│                                     ↑                       │"
  );
  linesChirho.push(
    "│                              rhema_ingest                    │"
  );
  linesChirho.push(
    "├──────────────────────────────────────────────────────────────┤"
  );
  linesChirho.push(
    "│                    rhema_contracts (types + traits)          │"
  );
  linesChirho.push(
    "├──────────────────────────────────────────────────────────────┤"
  );
  linesChirho.push(
    "│              rsword_chirho (SWORD binary compat)             │"
  );
  linesChirho.push(
    "└──────────────────────────────────────────────────────────────┘"
  );
  linesChirho.push("```");
  linesChirho.push("");

  // Data flow
  linesChirho.push("## Data Flow");
  linesChirho.push("");
  linesChirho.push("```");
  linesChirho.push(
    'User Query → parser_chirho → QueryNodeChirho (IR) → planner_chirho → PlanStepChirho → executor_chirho → QueryResultChirho'
  );
  linesChirho.push("              ↓                                        ↓");
  linesChirho.push(
    "         strong:, lemma:,                         Tantivy index"
  );
  linesChirho.push(
    '         NEAR/N, AND/OR/NOT                       or regex fallback'
  );
  linesChirho.push("```");
  linesChirho.push("");

  // Crate details
  linesChirho.push("## Crate Details");
  linesChirho.push("");

  const cratesChirho = manifestChirho.crate_chirho || [];

  const layerGroupsChirho: Record<string, CrateEntryChirho[]> = {};
  for (const crateChirho of cratesChirho) {
    const layerChirho = crateChirho.layer_chirho || "other";
    if (!layerGroupsChirho[layerChirho])
      layerGroupsChirho[layerChirho] = [];
    layerGroupsChirho[layerChirho].push(crateChirho);
  }

  const layerOrderChirho = [
    "core",
    "integration",
    "quality",
    "research",
  ];
  for (const layerChirho of layerOrderChirho) {
    const groupChirho = layerGroupsChirho[layerChirho];
    if (!groupChirho) continue;

    linesChirho.push(
      `### ${layerChirho.charAt(0).toUpperCase() + layerChirho.slice(1)} Layer`
    );
    linesChirho.push("");

    for (const crateChirho of groupChirho) {
      linesChirho.push(`#### \`${crateChirho.name_chirho}\``);
      linesChirho.push("");
      linesChirho.push(crateChirho.purpose_chirho);
      linesChirho.push("");

      if (crateChirho.modules_chirho?.length > 0) {
        linesChirho.push(
          `**Modules:** ${crateChirho.modules_chirho.join(", ")}`
        );
        linesChirho.push("");
      }

      const capsChirho = crateChirho.capability_chirho || [];
      if (capsChirho.length > 0) {
        linesChirho.push("**Capabilities:**");
        linesChirho.push("");
        for (const capChirho of capsChirho) {
          const statusChirho =
            capChirho.status_chirho === "implemented"
              ? "[x]"
              : capChirho.status_chirho === "stubbed"
                ? "[-]"
                : "[ ]";
          linesChirho.push(`- ${statusChirho} ${capChirho.name_chirho}`);
        }
        linesChirho.push("");
      }
    }
  }

  linesChirho.push(
    "---"
  );
  linesChirho.push(
    `*Generated by gen_docs_chirho.ts on ${new Date().toISOString().split("T")[0]}*`
  );

  return linesChirho.join("\n");
}

function generateSearchCapabilitiesChirho(
  manifestChirho: ManifestChirho
): string {
  const linesChirho: string[] = [];

  linesChirho.push(
    "<!-- For God so loved the world that he gave his only begotten Son,"
  );
  linesChirho.push(
    "     that whoever believes in him should not perish but have eternal life."
  );
  linesChirho.push("     John 3:16 -->");
  linesChirho.push("");
  linesChirho.push("# Search Capabilities Matrix (Auto-Generated)");
  linesChirho.push("");
  linesChirho.push(
    "This document tracks rhema_chirho search features vs Logos/Accordance/BibleArc parity."
  );
  linesChirho.push("");

  // Feature matrix
  linesChirho.push("## Feature Matrix");
  linesChirho.push("");
  linesChirho.push(
    "| Feature | Status | Parser | Planner | Executor | Tests | Notes |"
  );
  linesChirho.push(
    "|---------|--------|--------|---------|----------|-------|-------|"
  );

  const featuresChirho = manifestChirho.search_feature_chirho || [];
  for (const featChirho of featuresChirho) {
    const yesNoChirho = (boolChirho: boolean) => (boolChirho ? "yes" : "-");
    const noteChirho = featChirho.note_chirho || "";
    linesChirho.push(
      `| ${featChirho.name_chirho} | **${featChirho.status_chirho}** | ${yesNoChirho(featChirho.parser_chirho)} | ${yesNoChirho(featChirho.planner_chirho)} | ${yesNoChirho(featChirho.executor_chirho)} | ${yesNoChirho(featChirho.tests_chirho)} | ${noteChirho} |`
    );
  }
  linesChirho.push("");

  // Summary counts
  const implementedCountChirho = featuresChirho.filter(
    (fChirho) => fChirho.status_chirho === "implemented"
  ).length;
  const stubbedCountChirho = featuresChirho.filter(
    (fChirho) => fChirho.status_chirho === "stubbed"
  ).length;
  const plannedCountChirho = featuresChirho.filter(
    (fChirho) => fChirho.status_chirho === "planned"
  ).length;
  const totalCountChirho = featuresChirho.length;

  linesChirho.push("## Summary");
  linesChirho.push("");
  linesChirho.push(`- **Implemented:** ${implementedCountChirho}/${totalCountChirho}`);
  linesChirho.push(`- **Stubbed:** ${stubbedCountChirho}/${totalCountChirho}`);
  linesChirho.push(`- **Planned:** ${plannedCountChirho}/${totalCountChirho}`);
  linesChirho.push("");

  // Logos comparison
  linesChirho.push("## Logos/Accordance/BibleArc Comparison");
  linesChirho.push("");
  linesChirho.push("| Feature | Logos | Accordance | BibleArc | rhema_chirho |");
  linesChirho.push("|---------|-------|------------|----------|-------------|");
  linesChirho.push(
    "| Boolean Search | yes | yes | - | **implemented** |"
  );
  linesChirho.push(
    "| Phrase Search | yes | yes | - | **implemented** |"
  );
  linesChirho.push(
    "| Proximity | yes | yes | - | **implemented** |"
  );
  linesChirho.push(
    "| Strong's | yes | yes | - | **implemented** |"
  );
  linesChirho.push(
    "| Lemma | yes | yes | - | **implemented** |"
  );
  linesChirho.push(
    "| Morphology | yes | yes | - | planned |"
  );
  linesChirho.push(
    "| Semantic Domain | yes | yes | - | planned |"
  );
  linesChirho.push(
    "| Syntax Search | yes | yes | - | planned (Phase D) |"
  );
  linesChirho.push(
    "| Cross-references | yes | yes | - | planned |"
  );
  linesChirho.push(
    "| Discourse Analysis | - | - | yes | planned (Phase D) |"
  );
  linesChirho.push(
    "| Visual Query Builder | yes | yes | - | planned (Phase D) |"
  );
  linesChirho.push(
    "| Saved Searches | yes | yes | - | planned |"
  );
  linesChirho.push("");

  // Platform integrations
  linesChirho.push("## Platform Integrations");
  linesChirho.push("");
  linesChirho.push("| Platform | Method | Status | Feasibility | Notes |");
  linesChirho.push("|----------|--------|--------|-------------|-------|");

  const integrationsChirho = manifestChirho.integration_chirho || [];
  for (const intChirho of integrationsChirho) {
    const noteChirho = intChirho.note_chirho || "";
    linesChirho.push(
      `| ${intChirho.platform_chirho} | ${intChirho.method_chirho} | **${intChirho.status_chirho}** | ${intChirho.feasibility_chirho} | ${noteChirho} |`
    );
  }
  linesChirho.push("");

  // Open data sources
  linesChirho.push("## Open Data Sources for Advanced Search");
  linesChirho.push("");
  linesChirho.push("| Dataset | License | Purpose | Status |");
  linesChirho.push("|---------|---------|---------|--------|");
  linesChirho.push(
    "| Robinson RMAC | CC-BY-SA 3.0 | Greek morphology codes | Available via SWORD |"
  );
  linesChirho.push(
    "| OSHM codes | In SWORD modules | Hebrew morphology | Available via SWORD |"
  );
  linesChirho.push(
    "| MorphGNT/SBLGNT | CC-BY-SA | Full Greek morphological tagging | Not yet imported |"
  );
  linesChirho.push(
    "| OSHB/MorphHB | CC BY 4.0 | Complete Hebrew morphology | Not yet imported |"
  );
  linesChirho.push(
    "| STEP Bible TAGNT/TAHOT | CC BY 4.0 | Expanded morph codes | Not yet imported |"
  );
  linesChirho.push(
    "| SDGNT (Louw-Nida) | CC-BY-SA 4.0 | Semantic domains for Greek | Not yet imported |"
  );
  linesChirho.push(
    "| UBS Parallel Passages | CC-BY-SA 4.0 | Cross-reference data | Not yet imported |"
  );
  linesChirho.push(
    "| TSK | Public domain | Cross-references (via SWORD) | Not yet imported |"
  );
  linesChirho.push(
    "| semantic-chirho | Internal | Token-sense-domain pipeline | Planned bridge |"
  );
  linesChirho.push(
    "| ETCBC/BHSA | Research use | Hebrew syntax trees | Phase D |"
  );
  linesChirho.push(
    "| OpenText.org | Research | Greek linguistic annotations | Phase D |"
  );
  linesChirho.push("");

  linesChirho.push(
    "---"
  );
  linesChirho.push(
    `*Generated by gen_docs_chirho.ts on ${new Date().toISOString().split("T")[0]}*`
  );

  return linesChirho.join("\n");
}

// ─── Main ─────────────────────────────────────────────────────

function mainChirho(): void {
  const scriptDirChirho = dirname(Bun.main);
  const docsBaseDirChirho = join(scriptDirChirho, "..", "docs_chirho");
  const manifestPathChirho = join(docsBaseDirChirho, "manifest_chirho.toml");

  console.log(`Reading manifest: ${manifestPathChirho}`);

  let manifestContentChirho: string;
  try {
    manifestContentChirho = readFileSync(manifestPathChirho, "utf-8");
  } catch (errorChirho) {
    console.error(`Failed to read manifest: ${errorChirho}`);
    process.exit(1);
  }

  const manifestChirho = parseTomlChirho(manifestContentChirho);

  // Ensure output directory exists
  mkdirSync(docsBaseDirChirho, { recursive: true });

  // Generate AI Context
  const aiContextChirho = generateAiContextChirho(manifestChirho);
  const aiContextPathChirho = join(docsBaseDirChirho, "AI_CONTEXT_CHIRHO.md");
  writeFileSync(aiContextPathChirho, aiContextChirho);
  const aiLineCountChirho = aiContextChirho.split("\n").length;
  console.log(
    `Generated ${aiContextPathChirho} (${aiLineCountChirho} lines)`
  );

  // Generate Architecture
  const architectureChirho = generateArchitectureChirho(manifestChirho);
  const architecturePathChirho = join(
    docsBaseDirChirho,
    "ARCHITECTURE_CHIRHO.md"
  );
  writeFileSync(architecturePathChirho, architectureChirho);
  console.log(`Generated ${architecturePathChirho}`);

  // Generate Search Capabilities
  const searchCapsChirho = generateSearchCapabilitiesChirho(manifestChirho);
  const searchCapsPathChirho = join(
    docsBaseDirChirho,
    "SEARCH_CAPABILITIES_CHIRHO.md"
  );
  writeFileSync(searchCapsPathChirho, searchCapsChirho);
  console.log(`Generated ${searchCapsPathChirho}`);

  // Verify AI context file is under 500 lines
  if (aiLineCountChirho > 500) {
    console.warn(
      `WARNING: AI_CONTEXT_CHIRHO.md is ${aiLineCountChirho} lines (target: <500)`
    );
  } else {
    console.log(`AI context file is ${aiLineCountChirho} lines (within 500 limit)`);
  }

  console.log("Documentation generation complete.");
}

mainChirho();
