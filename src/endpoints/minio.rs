#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::{AoristConcept, AoristConceptChildren, ConceptEnum};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct MinioConfig {
    pub server: String,
    #[py_default = "9000"]
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
