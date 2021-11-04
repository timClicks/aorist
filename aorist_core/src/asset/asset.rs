#![allow(non_snake_case)]
use crate::asset::geospatial_asset::*;
use crate::asset::graph_asset::*;
use crate::asset::language_asset::*;
use crate::asset::static_data_table::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::encoding::{Encoding, PyEncoding};
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::asset_enum;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

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

asset_enum! {
    name: Asset
    concrete_variants:
    - StaticDataTable
    enum_variants:
    - GeospatialAsset
    - LanguageAsset
    - GraphAsset
}

impl Asset {
    pub fn persist_local(&self, persistent: AoristRef<Storage>) -> Self {
        let mut cloned = self.clone();
        let storage_setup = cloned.get_storage_setup();
        let new_setup = match *storage_setup.0.read().unwrap() {
            StorageSetup::LocalStorageSetup(_) => AoristRef(Arc::new(RwLock::new(
                cloned
                    .get_storage_setup()
                    .0
                    .read()
                    .unwrap()
                    .persist_local(persistent),
            ))),
            _ => cloned.get_storage_setup(),
        };
        cloned.set_storage_setup(new_setup);
        cloned
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyAsset {
    pub fn persist_local(&self, storage: PyStorage) -> PyResult<Self> {
        Ok(PyAsset{ 
            inner: AoristRef(Arc::new(RwLock::new(
                self.inner.0.read().unwrap().persist_local(storage.inner.deep_clone())
            )))
        })
    }
    pub fn replicate_to_local(
        &self,
        storage: PyStorage,
        tmp_dir: String,
        tmp_encoding: PyEncoding,
    ) -> PyResult<Self> {
        Ok(PyAsset { inner: AoristRef(Arc::new(RwLock::new(self.inner.0.read().unwrap().replicate_to_local(
            storage.inner.deep_clone(),
            tmp_dir.clone(),
            tmp_encoding.inner.deep_clone(),
        ).unwrap()))) } ) 
    }
}
