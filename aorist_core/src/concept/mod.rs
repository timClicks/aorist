// pub use crate::universe::*;
pub use aorist_primitives::{register_concept, Ancestry, AoristConcept, ConceptEnum, TConceptEnum};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, RwLock};
use tracing::debug;
use uuid::Uuid;

use serde::{Serialize, Deserialize};
use aorist_concept::{aorist2, Constrainable};
use derivative::Derivative;
use paste::paste;
#[aorist2]
pub struct GlobalPermissionsAdmin {}
#[aorist2]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
#[aorist2]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    pub unixname: String,
    #[constrainable2]
    roles: Option<Vec<Role>>,
}

// use crate::access_policy::*;
// use crate::algorithms::*;
// use crate::asset::*;
// use crate::attributes::*;
// use crate::compliance::*;
// use crate::compression::*;
// use crate::dataset::*;
// use crate::encoding::*;
// use crate::endpoints::*;
// use crate::header::*;
// use crate::layout::*;
// use crate::location::*;
// use crate::models::*;
// use crate::predicate::*;
// use crate::role::*;
// use crate::role_binding::*;
// use crate::schema::*;
// use crate::storage::*;
// use crate::storage_setup::*;
// use crate::template::*;
// use crate::user::*;
// use crate::user_group::*;
// pub struct WrappedConcept<'a, T> {
//     pub inner: T,
//     pub _phantom_lt: std::marker::PhantomData<&'a ()>,
// }
// 
// register_concept!(
//     Concept,
//     ConceptAncestry,
//     AccessPolicy,
//     ApproveAccessSelector,
//     RegressionAlgorithm,
//     RandomForestRegressionAlgorithm,
//     SVMRegressionAlgorithm,
//     Asset,
//     StaticDataTable,
//     SupervisedModel,
//     DerivedAsset,
//     Attribute,
//     Predicate,
//     APILayout,
//     APIOrFileLayout,
//     FileBasedStorageLayout,
//     SingleFileLayout,
//     PushshiftSubredditPostsAPILayout,
//     TabularLayout,
//     DynamicTabularLayout,
//     StaticTabularLayout,
//     Granularity,
//     DailyGranularity,
//     DataSet,
//     Role,
//     GlobalPermissionsAdmin,
//     GzipCompression,
//     DataCompression,
//     ZipCompression,
//     ComplianceConfig,
//     CSVHeader,
//     FileHeader,
//     AlluxioLocation,
//     BigQueryLocation,
//     GCSLocation,
//     GithubLocation,
//     HiveLocation,
//     LocalFileSystemLocation,
//     OnPremiseLocation,
//     MinioLocation,
//     S3Location,
//     PostgresLocation,
//     PushshiftAPILocation,
//     RemoteLocation,
//     SQLiteLocation,
//     WebLocation,
//     Model,
//     SingleObjectiveRegressor,
//     GDBEncoding,
//     CSVEncoding,
//     TSVEncoding,
//     Encoding,
//     JSONEncoding,
//     NewlineDelimitedJSONEncoding,
//     ORCEncoding,
//     ONNXEncoding,
//     UndefinedTabularSchema,
//     TabularSchema,
//     TimeOrderedTabularSchema,
//     DataSchema,
//     Universe,
//     LocalStorageSetup,
//     RemoteStorageSetup,
//     ReplicationStorageSetup,
//     ComputedFromLocalData,
//     StorageSetup,
//     Storage,
//     BigQueryStorage,
//     SQLiteStorage,
//     HiveTableStorage,
//     RemoteStorage,
//     LocalFileStorage,
//     PostgresStorage,
//     GitStorage,
//     RoleBinding,
//     DatumTemplate,
//     IdentifierTuple,
//     RowStruct,
//     IntegerMeasure,
//     Filter,
//     User,
//     UserGroup,
//     EndpointConfig,
//     AWSConfig,
//     GCPConfig,
//     GiteaConfig,
//     PostgresConfig,
//     AlluxioConfig,
//     RangerConfig,
//     PrestoConfig,
//     MinioConfig,
//     TrainedFloatMeasure,
//     PredictionsFromTrainedFloatMeasure
// );
