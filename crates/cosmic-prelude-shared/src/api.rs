use actionable::Action;
use bonsaidb_core::{connection::SensitiveString, custom_api::CustomApi};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Api;

impl CustomApi for Api {
    type Request = Request;
    type Response = Response;
    type Error = Error;
}

#[derive(Debug, Serialize, Deserialize, actionable::Actionable)]
pub enum Request {
    #[actionable(protection = "none")]
    Register {
        username: String,
        password: SensitiveString,
    },
    #[actionable(protection = "simple")]
    CreateCharacter { name: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Response {
    Ok,
}

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    #[error("invalid character name")]
    InvalidCharacterName,
    #[error("character name taken")]
    CharacterNameTaken,
}

#[derive(Action, Debug)]
pub enum Prelude {
    CreateCharacter,
}
