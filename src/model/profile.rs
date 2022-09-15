use serde::{ser::SerializeStruct, Serialize};
use uuid::Uuid;

// #[derive(Serialize)]
pub struct Profile {
    id: Uuid,
    name: String,
    textures: Textures,
}

impl Serialize for Profile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Profile", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

pub struct Textures {
    // todo!()
}
