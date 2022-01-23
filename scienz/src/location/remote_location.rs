use crate::location::bigquery_location::*;
use crate::location::gcs_location::*;
use crate::location::github_location::*;
use crate::location::pushshift_api_location::*;
use crate::location::s3_location::*;
use crate::location::web_location::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub enum RemoteLocation {
    #[constrainable]
    GCSLocation(AoristRef<GCSLocation>),
    #[constrainable]
    WebLocation(AoristRef<WebLocation>),
    #[constrainable]
    PushshiftAPILocation(AoristRef<PushshiftAPILocation>),
    #[constrainable]
    BigQueryLocation(AoristRef<BigQueryLocation>),
    #[constrainable]
    GithubLocation(AoristRef<GithubLocation>),
    #[constrainable]
    S3Location(AoristRef<S3Location>),
}
