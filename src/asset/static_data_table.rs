#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectAsset, TPrefectStorageSetup};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct StaticDataTable {
    pub name: String,
    #[constrainable]
    setup: StorageSetup,
    #[constrainable]
    schema: DataSchema,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl StaticDataTable {
    pub fn get_presto_schemas(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> String {
        let columnSchema = self.schema.get_presto_schema(templates);
        self.setup
            .get_presto_schemas(self.get_name(), columnSchema, endpoints)
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
impl TObjectWithPythonCodeGen for StaticDataTable {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.setup.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for StaticDataTable {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.setup.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectAsset for StaticDataTable {
    fn get_prefect_dag(
        &self,
        templates: &HashMap<String, DatumTemplate>,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        self.setup
            .get_prefect_dag(&self.schema, templates, self.get_name(), endpoints)
    }
}
