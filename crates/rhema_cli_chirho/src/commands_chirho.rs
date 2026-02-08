// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! CLI command implementations.

use rhema_exec_chirho::QueryExecutorImplChirho;
use rhema_index_chirho::ModuleIndexerChirho;
use rhema_ingest_chirho::SwordAdapterChirho;
use rhema_integration_chirho::RhemaLibraryChirho;
use rhema_query_chirho::QueryParserChirho;
use rhema_query_chirho::planner_chirho::QueryPlannerChirho;

/// Execute the `modules` command — list available modules.
pub fn cmd_modules_chirho(
    lib_chirho: &RhemaLibraryChirho,
    type_filter_chirho: Option<&str>,
) -> anyhow::Result<()> {
    let modules_chirho = match type_filter_chirho {
        Some("bible") | Some("bibles") => lib_chirho.list_bibles_chirho(),
        Some("commentary") | Some("commentaries") => lib_chirho.list_commentaries_chirho(),
        Some("lexicon") | Some("lexicons") | Some("dictionary") => {
            lib_chirho.list_lexicons_chirho()
        }
        _ => lib_chirho.list_all_modules_chirho(),
    };

    if modules_chirho.is_empty() {
        println!("No modules found.");
        return Ok(());
    }

    println!("{:<20} {:<15} {:<6} {}", "Name", "Type", "Lang", "Description");
    println!("{}", "-".repeat(70));

    for mod_chirho in &modules_chirho {
        println!(
            "{:<20} {:<15} {:<6} {}",
            mod_chirho.name_chirho,
            mod_chirho.module_type_chirho,
            mod_chirho.language_chirho,
            truncate_chirho(&mod_chirho.description_chirho, 30),
        );
    }

    println!("\nTotal: {} modules", modules_chirho.len());
    Ok(())
}

/// Execute the `info` command — display module details.
pub fn cmd_info_chirho(
    lib_chirho: &RhemaLibraryChirho,
    module_name_chirho: &str,
) -> anyhow::Result<()> {
    match lib_chirho.get_module_info_chirho(module_name_chirho) {
        Some(info_chirho) => {
            println!("Module: {}", info_chirho.name_chirho);
            println!("Type: {}", info_chirho.module_type_chirho);
            println!("Language: {}", info_chirho.language_chirho);
            println!("Versification: {}", info_chirho.versification_chirho);
            println!("Description: {}", info_chirho.description_chirho);
        }
        None => {
            println!("Module '{}' not found.", module_name_chirho);
        }
    }
    Ok(())
}

/// Execute the `lookup` command — read verses/chapters.
pub fn cmd_lookup_chirho(
    lib_chirho: &RhemaLibraryChirho,
    module_name_chirho: &str,
    book_chirho: &str,
    chapter_chirho: u16,
    verse_chirho: Option<u16>,
) -> anyhow::Result<()> {
    if let Some(v_chirho) = verse_chirho {
        // Single verse
        let result_chirho =
            lib_chirho.read_verse_chirho(module_name_chirho, book_chirho, chapter_chirho, v_chirho)?;
        println!(
            "{} {}:{} ({})",
            result_chirho.book_chirho,
            result_chirho.chapter_chirho,
            result_chirho.verse_chirho,
            module_name_chirho
        );
        println!("{}", strip_tags_chirho(&result_chirho.text_chirho));
    } else {
        // Whole chapter
        let chapter_result_chirho =
            lib_chirho.read_chapter_chirho(module_name_chirho, book_chirho, chapter_chirho)?;

        println!(
            "{} {} ({})",
            chapter_result_chirho.book_chirho,
            chapter_result_chirho.chapter_chirho,
            module_name_chirho,
        );
        println!();

        for verse_chirho in &chapter_result_chirho.verses_chirho {
            if !verse_chirho.text_chirho.is_empty() {
                println!(
                    "  {}:{} {}",
                    verse_chirho.chapter_chirho,
                    verse_chirho.verse_chirho,
                    strip_tags_chirho(&verse_chirho.text_chirho)
                );
            }
        }
    }
    Ok(())
}

/// Execute the `search` command — run a query.
pub fn cmd_search_chirho(
    module_name_chirho: &str,
    query_text_chirho: &str,
    max_results_chirho: usize,
    explain_chirho: bool,
) -> anyhow::Result<()> {
    let mut query_chirho = QueryParserChirho::parse_chirho(query_text_chirho)?;
    query_chirho = query_chirho.with_max_results_chirho(max_results_chirho);
    if explain_chirho {
        query_chirho = query_chirho.with_explain_chirho();
    }

    let executor_chirho = QueryExecutorImplChirho::with_system_paths_chirho()
        .map_err(|e_chirho| anyhow::anyhow!("{}", e_chirho))?;

    let result_chirho = executor_chirho
        .execute_against_module_chirho(&query_chirho, module_name_chirho)
        .map_err(|e_chirho| anyhow::anyhow!("{}", e_chirho))?;

    println!(
        "Search: \"{}\" in {} — {} hits ({} us)",
        query_text_chirho,
        module_name_chirho,
        result_chirho.total_count_chirho,
        result_chirho.execution_time_us_chirho,
    );

    if let Some(plan_chirho) = &result_chirho.plan_chirho {
        println!("\nExecution plan:\n{}", plan_chirho);
    }

    println!();
    for (i_chirho, hit_chirho) in result_chirho.hits_chirho.iter().enumerate() {
        println!(
            "  {}. {} — {}",
            i_chirho + 1,
            hit_chirho.verse_ref_chirho,
            truncate_chirho(&strip_tags_chirho(&hit_chirho.text_chirho), 80),
        );
    }

    Ok(())
}

/// Execute the `parse` command — parse and explain a query.
pub fn cmd_parse_chirho(query_text_chirho: &str) -> anyhow::Result<()> {
    let query_chirho = QueryParserChirho::parse_chirho(query_text_chirho)?;
    let plan_chirho = QueryPlannerChirho::plan_chirho(&query_chirho);

    println!("Query: \"{}\"", query_text_chirho);
    println!();
    println!("Parsed IR:");
    println!("  {:?}", query_chirho.root_chirho);

    if let Some(scope_chirho) = &query_chirho.scope_chirho {
        println!("  Scope: {:?}", scope_chirho);
    }

    println!();
    println!("Execution plan:");
    println!("  {}", QueryPlannerChirho::explain_chirho(&plan_chirho));

    Ok(())
}

/// Execute the `index` command — build or delete a search index.
pub fn cmd_index_chirho(
    module_name_chirho: &str,
    delete_chirho: bool,
) -> anyhow::Result<()> {
    let index_base_chirho = std::env::var("HOME")
        .map(|h_chirho| {
            std::path::PathBuf::from(h_chirho)
                .join(".sword")
                .join("rhema_indexes")
        })
        .map_err(|_| anyhow::anyhow!("HOME not set"))?;

    let indexer_chirho = ModuleIndexerChirho::new_chirho(&index_base_chirho);

    if delete_chirho {
        if indexer_chirho.has_index_chirho(module_name_chirho) {
            indexer_chirho
                .delete_index_chirho(module_name_chirho)
                .map_err(|e_chirho| anyhow::anyhow!("{}", e_chirho))?;
            println!("Deleted index for '{}'.", module_name_chirho);
        } else {
            println!("No index found for '{}'.", module_name_chirho);
        }
        return Ok(());
    }

    let adapter_chirho = SwordAdapterChirho::with_system_paths_chirho()?;
    if !adapter_chirho.has_module_chirho(module_name_chirho) {
        println!("Module '{}' not found.", module_name_chirho);
        return Ok(());
    }

    println!("Building search index for '{}'...", module_name_chirho);
    let progress_chirho = |books_done_chirho: usize,
                           total_books_chirho: usize,
                           verses_chirho: usize| {
        print!(
            "\r  [{}/{}] books, {} verses indexed",
            books_done_chirho, total_books_chirho, verses_chirho,
        );
    };

    let count_chirho = indexer_chirho
        .build_index_chirho(
            &adapter_chirho,
            module_name_chirho,
            Some(Box::new(progress_chirho)),
        )
        .map_err(|e_chirho| anyhow::anyhow!("{}", e_chirho))?;

    println!();
    println!(
        "Index built: {} verses indexed for '{}'.",
        count_chirho, module_name_chirho,
    );
    println!(
        "Index path: {}",
        indexer_chirho.index_path_chirho(module_name_chirho).display(),
    );

    Ok(())
}

/// Strip HTML/XML tags for plain text output.
fn strip_tags_chirho(text_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(text_chirho.len());
    let mut in_tag_chirho = false;

    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '<' => in_tag_chirho = true,
            '>' => in_tag_chirho = false,
            _ if !in_tag_chirho => result_chirho.push(ch_chirho),
            _ => {}
        }
    }

    result_chirho
}

/// Truncate a string to max length with ellipsis.
fn truncate_chirho(text_chirho: &str, max_len_chirho: usize) -> String {
    if text_chirho.len() <= max_len_chirho {
        text_chirho.to_string()
    } else {
        format!("{}...", &text_chirho[..max_len_chirho.saturating_sub(3)])
    }
}
