use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct RemoteStorage {
    #[constrainable]
    pub location: AoristRef<RemoteLocation>,
    #[constrainable]
    pub layout: AoristRef<APIOrFileLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
