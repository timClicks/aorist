#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, WrappedConcept, ConceptEnum, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct RandomForestRegressionAlgorithm {}
#[aorist_concept]
pub struct SVMRegressionAlgorithm {}

#[aorist_concept]
pub enum RegressionAlgorithm {
    RandomForestRegressionAlgorithm(RandomForestRegressionAlgorithm),
    SVMRegressionAlgorithm(SVMRegressionAlgorithm),
}
