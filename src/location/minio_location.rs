use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct MinioLocation {
    pub name: String,
}
