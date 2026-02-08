// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Discourse storage trait — defines the persistence contract.

use crate::error_chirho::DiscourseResultChirho;
use crate::types_chirho::{ArcIdChirho, ArcStructureChirho, RelationshipTypeChirho};

/// Trait for discourse analysis storage implementations.
pub trait DiscourseStoreChirho {
    /// Save an arc structure (insert or update).
    fn save_arc_chirho(&self, arc_chirho: &ArcStructureChirho) -> DiscourseResultChirho<ArcIdChirho>;

    /// Load an arc structure by ID.
    fn load_arc_chirho(&self, id_chirho: ArcIdChirho) -> DiscourseResultChirho<ArcStructureChirho>;

    /// Delete an arc structure.
    fn delete_arc_chirho(&self, id_chirho: ArcIdChirho) -> DiscourseResultChirho<()>;

    /// List all arc structures.
    fn list_arcs_chirho(&self) -> DiscourseResultChirho<Vec<(ArcIdChirho, String)>>;

    /// Find arcs containing a specific relationship type.
    fn find_by_relationship_chirho(
        &self,
        relationship_type_chirho: RelationshipTypeChirho,
    ) -> DiscourseResultChirho<Vec<ArcIdChirho>>;

    /// Find arcs containing proposition text matching a query.
    fn find_by_proposition_text_chirho(
        &self,
        text_query_chirho: &str,
    ) -> DiscourseResultChirho<Vec<ArcIdChirho>>;

    /// Get the count of stored arcs.
    fn count_arcs_chirho(&self) -> DiscourseResultChirho<u64>;
}
