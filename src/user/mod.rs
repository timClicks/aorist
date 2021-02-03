#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::error::AoristError;
use crate::object::TAoristObject;
use crate::role::{Role, TRole};
use aorist_concept::{Constrainable, aorist_concept};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug, Hash, Eq, Clone)]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
    roles: Option<Vec<Role>>,
}

pub trait TUser {
    fn to_yaml(&self) -> String;
    fn get_unixname(&self) -> String;
    fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), AoristError>;
    fn get_roles(&self) -> Result<Vec<Role>, AoristError>;
    fn get_permissions(&self) -> Result<HashSet<String>, AoristError>;
}
impl TUser for User {
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    fn get_unixname(&self) -> String {
        self.unixname.clone()
    }
    fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), AoristError> {
        if let Some(_) = self.roles {
            return Err(AoristError::OtherError(
                "Tried to set roles more than once.".to_string(),
            ));
        }
        self.roles = Some(roles);
        Ok(())
    }
    fn get_roles(&self) -> Result<Vec<Role>, AoristError> {
        match &self.roles {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get roles for user but set_roles was never called".to_string(),
            )),
        }
    }
    fn get_permissions(&self) -> Result<HashSet<String>, AoristError> {
        let mut perms: HashSet<String> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
    }
}

#[pymethods]
impl User {
    #[new]
    #[args(phone = "\"\".to_string()", roles = "None")]
    fn new(
        firstName: String,
        lastName: String,
        email: String,
        phone: String,
        unixname: String,
        roles: Option<Vec<Role>>,
    ) -> Self {
        Self {
            firstName,
            lastName,
            email,
            phone,
            unixname,
            roles,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}
