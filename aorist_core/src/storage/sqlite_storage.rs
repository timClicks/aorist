use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct SQLiteStorage {
    #[constrainable]
    pub location: AoristRef<SQLiteLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
}
