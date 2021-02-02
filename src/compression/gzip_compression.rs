#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct GzipCompression {
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
#[pymethods]
impl GzipCompression {
    #[new]
    fn new() -> Self {
        Self {
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
