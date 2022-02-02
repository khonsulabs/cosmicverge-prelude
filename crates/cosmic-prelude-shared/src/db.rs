use bonsaidb_core::schema::{Schema, SchemaName, Schematic};

pub mod character;

#[derive(Debug)]
pub enum PreludeSchema {}

impl Schema for PreludeSchema {
    fn schema_name() -> SchemaName {
        SchemaName::new("khonsulabs", "cosmic-prelude")
    }

    fn define_collections(schema: &mut Schematic) -> Result<(), bonsaidb_core::Error> {
        schema.define_collection::<character::Character>()?;
        Ok(())
    }
}
