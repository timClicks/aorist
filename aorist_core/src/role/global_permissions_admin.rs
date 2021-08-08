use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::role::role::TRole;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct GlobalPermissionsAdmin {}
impl TRole for AoristRef<GlobalPermissionsAdmin> {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
