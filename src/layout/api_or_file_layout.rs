#![allow(non_snake_case)]
use super::api_layout::*;
use super::file_based_storage_layout::*;
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum APIOrFileLayout {
    FileBasedStorageLayout(FileBasedStorageLayout),
    APILayout(APILayout),
}
