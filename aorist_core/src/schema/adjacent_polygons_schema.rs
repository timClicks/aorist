use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConceptBase, AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: AdjacentPolygonsSchema,
    source: PolygonCollectionAsset,
    attributes:
      id1: KeyInt64Identifier("Polygon 1 Identifier", false),
      id2: KeyInt64Identifier("Polygon 2 Identifier", false)
    fields:
      buffer: AOption<FloatValue>
}
