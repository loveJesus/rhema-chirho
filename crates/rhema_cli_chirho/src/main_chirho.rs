// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Rhema Chirho CLI — Biblical scholarship engine command-line interface.

use clap::{Parser, Subcommand};
use rhema_integration_chirho::RhemaLibraryChirho;

use rhema_cli_chirho::commands_chirho;

/// Rhema Chirho — Biblical scholarship engine CLI
#[derive(Parser)]
#[command(
    name = "rhema_chirho",
    version,
    about = "Rhema Chirho — Biblical scholarship engine in Pure Rust",
    long_about = "A powerful biblical text engine supporting SWORD modules,\n\
                  boolean/phrase/morphology search, Strong's concordance,\n\
                  and multi-backend query execution."
)]
struct CliChirho {
    #[command(subcommand)]
    command_chirho: CommandChirho,
}

#[derive(Subcommand)]
enum CommandChirho {
    /// List available modules
    Modules {
        /// Filter by type: bibles, commentaries, lexicons
        #[arg(short = 't', long = "type")]
        type_filter_chirho: Option<String>,
    },

    /// Display module information
    Info {
        /// Module name (e.g., KJV, ESV2011)
        #[arg(required = true)]
        module_chirho: String,
    },

    /// Look up a verse or chapter
    Lookup {
        /// Module name
        #[arg(short = 'b', long = "bible", default_value = "KJV")]
        module_chirho: String,

        /// Book name (e.g., John, Genesis, "I Corinthians")
        #[arg(required = true)]
        book_chirho: String,

        /// Chapter number
        #[arg(required = true)]
        chapter_chirho: u16,

        /// Verse number (omit for whole chapter)
        verse_chirho: Option<u16>,
    },

    /// Search within a module
    Search {
        /// Module name
        #[arg(short = 'b', long = "bible", default_value = "KJV")]
        module_chirho: String,

        /// Query string (supports AND/OR/NOT, phrases, strong:G26, lemma:agape)
        #[arg(required = true)]
        query_chirho: String,

        /// Maximum results
        #[arg(short = 'n', long = "max", default_value = "25")]
        max_results_chirho: usize,

        /// Show execution plan
        #[arg(long = "explain")]
        explain_chirho: bool,

        /// Save this search with a name for later re-use
        #[arg(long = "save")]
        save_chirho: Option<String>,

        /// Load and execute a previously saved search by name
        #[arg(long = "load")]
        load_chirho: Option<String>,
    },

    /// Manage saved searches
    SavedSearches {
        /// List all saved searches
        #[arg(short = 'l', long = "list")]
        list_chirho: bool,

        /// Filter by tag
        #[arg(short = 't', long = "tag")]
        tag_chirho: Option<String>,
    },

    /// Parse and explain a query without executing
    Parse {
        /// Query string
        #[arg(required = true)]
        query_chirho: String,
    },

    /// Build or manage search indexes for fast queries
    Index {
        /// Module name to index (e.g., KJV)
        #[arg(required = true)]
        module_chirho: String,

        /// Delete the existing index instead of building
        #[arg(short = 'd', long = "delete")]
        delete_chirho: bool,
    },

    /// Import semantic domain data from a semantic-chirho database
    ImportDomains {
        /// Path to the semantic-chirho SQLite database
        #[arg(required = true)]
        source_chirho: String,

        /// Output path for the domain store (default: data_chirho/domains.db)
        #[arg(short = 'o', long = "output", default_value = "data_chirho/domains.db")]
        output_chirho: String,
    },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli_chirho = CliChirho::parse();

    match cli_chirho.command_chirho {
        CommandChirho::Modules { type_filter_chirho } => {
            let lib_chirho = RhemaLibraryChirho::init_chirho()?;
            commands_chirho::cmd_modules_chirho(
                &lib_chirho,
                type_filter_chirho.as_deref(),
            )?;
        }

        CommandChirho::Info { module_chirho } => {
            let lib_chirho = RhemaLibraryChirho::init_chirho()?;
            commands_chirho::cmd_info_chirho(&lib_chirho, &module_chirho)?;
        }

        CommandChirho::Lookup {
            module_chirho,
            book_chirho,
            chapter_chirho,
            verse_chirho,
        } => {
            let lib_chirho = RhemaLibraryChirho::init_chirho()?;
            commands_chirho::cmd_lookup_chirho(
                &lib_chirho,
                &module_chirho,
                &book_chirho,
                chapter_chirho,
                verse_chirho,
            )?;
        }

        CommandChirho::Search {
            module_chirho,
            query_chirho,
            max_results_chirho,
            explain_chirho,
            save_chirho,
            load_chirho,
        } => {
            commands_chirho::cmd_search_chirho(
                &module_chirho,
                &query_chirho,
                max_results_chirho,
                explain_chirho,
            )?;
            if let Some(name_chirho) = save_chirho {
                commands_chirho::cmd_save_search_chirho(&name_chirho, &query_chirho)?;
            }
            if let Some(name_chirho) = load_chirho {
                commands_chirho::cmd_load_search_chirho(&name_chirho)?;
            }
        }

        CommandChirho::SavedSearches {
            list_chirho,
            tag_chirho,
        } => {
            commands_chirho::cmd_list_saved_searches_chirho(list_chirho, tag_chirho.as_deref())?;
        }

        CommandChirho::Parse { query_chirho } => {
            commands_chirho::cmd_parse_chirho(&query_chirho)?;
        }

        CommandChirho::Index {
            module_chirho,
            delete_chirho,
        } => {
            commands_chirho::cmd_index_chirho(&module_chirho, delete_chirho)?;
        }

        CommandChirho::ImportDomains {
            source_chirho,
            output_chirho,
        } => {
            commands_chirho::cmd_import_domains_chirho(&source_chirho, &output_chirho)?;
        }
    }

    Ok(())
}
