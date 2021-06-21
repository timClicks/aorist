use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct RandomForestRegressionAlgorithm {}
#[aorist]
pub struct SVMRegressionAlgorithm {}

#[aorist]
pub enum RegressionAlgorithm {
    RandomForestRegressionAlgorithm(AoristRef<RandomForestRegressionAlgorithm>),
    SVMRegressionAlgorithm(AoristRef<SVMRegressionAlgorithm>),
}
