#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::AoristConstraint;
use crate::constraint::Constraint;
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct RemoteStorage {
    #[constrainable]
    location: RemoteLocation,
    #[constrainable]
    layout: FileBasedStorageLayout,
    #[constrainable]
    encoding: Encoding,
}
