#![allow(non_snake_case)]
use crate::asset::derived_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::supervised_model::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use crate::encoding::Encoding;
use crate::storage::Storage;

#[aorist]
pub enum Asset {
    #[constrainable]
    DerivedAsset(AoristRef<DerivedAsset>),
    #[constrainable]
    StaticDataTable(AoristRef<StaticDataTable>),
    #[constrainable]
    SupervisedModel(AoristRef<SupervisedModel>),
}

pub trait TAsset {
    fn get_name(&self) -> String;
    fn get_schema(&self) -> AoristRef<DataSchema>;
    fn get_storage_setup(&self) -> AoristRef<StorageSetup>;
    fn get_template_name(&self) -> String {
        self.get_schema()
            .0
            .read()
            .unwrap()
            .get_datum_template_name()
            .unwrap()
    }
}

impl Asset {
    pub fn get_type(&self) -> String {
        match self {
            Asset::StaticDataTable(_) => "StaticDataTable",
            Asset::SupervisedModel(_) => "SupervisedModel",
            Asset::DerivedAsset(_) => "DerivedAsset",
        }
        .to_string()
    }
    pub fn get_name(&self) -> String {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().name.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().name.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().name.clone(),
        }
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().schema.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().schema.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().setup.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().setup.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().setup.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        match self {
            Asset::StaticDataTable(x) => {
                Asset::StaticDataTable(AoristRef(Arc::new(RwLock::new(
                    x.0.read().unwrap().replicate_to_local(
                        t, 
                        tmp_dir, 
                        tmp_encoding
                    )
                ))))
            }
            Asset::SupervisedModel(x) => {
                Asset::SupervisedModel(AoristRef(Arc::new(RwLock::new(
                    x.0.read().unwrap().replicate_to_local(t, tmp_dir, tmp_encoding)
                ))))
            }
            Asset::DerivedAsset(_) => panic!(
                "DerivedAssets are already stored locally, they cannot be replicated to local"
            ),
        }
    }
}
