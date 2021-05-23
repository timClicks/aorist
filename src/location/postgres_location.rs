use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct PostgresLocation {
    pub database: String,
    pub schema: String,
}
