use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use crate::location::bigquery_location::*;
use crate::location::gcs_location::*;
use crate::location::github_location::*;
use crate::location::pushshift_api_location::*;
use crate::location::web_location::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(GCSLocation),
    #[constrainable]
    WebLocation(WebLocation),
    #[constrainable]
    PushshiftAPILocation(PushshiftAPILocation),
    #[constrainable]
    BigQueryLocation(BigQueryLocation),
    #[constrainable]
    GithubLocation(GithubLocation),
}
