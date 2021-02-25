use crate::code_block::CodeBlock;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::python::PythonStatementInput;
use crate::python::{Assignment, Dict, Import, ParameterTuple, Preamble, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use uuid::Uuid;

pub struct ConstraintBlock<'a, T>
where
    T: ETLSingleton + 'a,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<CodeBlock<'a, T>>,
    singleton_type: PhantomData<T>,
    tasks_dict: Option<AST>,
}
impl<'a, T> ConstraintBlock<'a, T>
where
    T: ETLSingleton,
{
    pub fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<CodeBlock<'a, T>>,
        tasks_dict: Option<AST>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
            singleton_type: PhantomData,
            tasks_dict,
        }
    }
    pub fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|x| x.get_identifiers().into_iter())
            .flatten()
            .collect()
    }
    pub fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
    pub fn get_task_val_assignments(&'a self) -> Vec<AST> {
        match &self.tasks_dict {
            Some(ref val) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))],
            None => vec![],
        }
    }
    pub fn get_statements(
        &'a self,
        endpoints: &EndpointConfig,
        identifiers: &HashMap<Uuid, AST>,
    ) -> PythonStatementInput {
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements(endpoints, identifiers))
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<Preamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        (
            self.get_task_val_assignments()
                .into_iter()
                .chain(preambles_and_statements.into_iter().map(|x| x.0).flatten())
                .collect::<Vec<_>>(),
            preambles,
            imports,
            self.constraint_name.clone(),
            self.title.clone(),
            self.body.clone(),
        )
    }
}
