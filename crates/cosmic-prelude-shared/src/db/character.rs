use bonsaidb_core::{
    document::CollectionDocument,
    schema::{
        Collection, CollectionName, CollectionViewSchema, DefaultSerialization,
        DefaultViewSerialization, Name, Schematic, View, ViewMapResult,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub user_id: u64,
    pub name: String,
}

impl Character {
    pub fn normalize_name(name: &str) -> String {
        name.trim().to_ascii_lowercase()
    }
}

impl Collection for Character {
    fn collection_name() -> CollectionName {
        CollectionName::new("khonsulabs", "character")
    }

    fn define_views(schema: &mut Schematic) -> Result<(), bonsaidb_core::Error> {
        schema.define_view(ByName)?;
        schema.define_view(ByUserId)?;
        Ok(())
    }
}

impl DefaultSerialization for Character {}

#[derive(Debug, Clone)]
pub struct ByName;

impl View for ByName {
    type Collection = Character;
    type Key = String;
    type Value = ();

    fn name(&self) -> Name {
        Name::new("by-name")
    }
}

impl DefaultViewSerialization for ByName {}

impl CollectionViewSchema for ByName {
    type View = ByName;

    fn unique(&self) -> bool {
        false
    }

    fn map(&self, document: CollectionDocument<Character>) -> ViewMapResult<Self::View> {
        Ok(document
            .header
            .emit_key(Character::normalize_name(&document.contents.name)))
    }
}

#[derive(Debug, Clone)]
pub struct ByUserId;

impl View for ByUserId {
    type Collection = Character;
    type Key = u64;
    type Value = u32;

    fn name(&self) -> Name {
        Name::new("by-user-id")
    }
}

impl CollectionViewSchema for ByUserId {
    type View = Self;

    fn map(&self, document: CollectionDocument<Character>) -> ViewMapResult<Self::View> {
        Ok(document
            .header
            .emit_key_and_value(document.contents.user_id, 1))
    }
}

impl DefaultViewSerialization for ByUserId {}
