use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::etl_task::{ETLTask, ForLoopETLTask, StandaloneETLTask};
use crate::python::{
    Import, ParameterTuple, Preamble, SimpleIdentifier, StringLiteral, Subscript, AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct CodeBlock<'a, T>
where
    T: ETLSingleton + 'a,
{
    _dialect: Option<Dialect>,
    members: Vec<(AST, Arc<RwLock<ConstraintState<'a>>>)>,
    tasks_dict: Option<AST>,
    constraint_name: String,
    singleton_type: PhantomData<T>,
}
impl<'a, T> CodeBlock<'a, T>
where
    T: ETLSingleton,
{
    pub fn get_tasks_dict(&self) -> Option<AST> {
        self.tasks_dict.clone()
    }
    pub fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|(x, rw)| (rw.read().unwrap().get_constraint_uuid(), x.clone()))
            .collect()
    }
    pub fn new(
        dialect: Option<Dialect>,
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
    ) -> Self {
        Self {
            _dialect: dialect,
            members: Self::compute_task_vals(members, &constraint_name, &tasks_dict),
            constraint_name,
            singleton_type: PhantomData,
            tasks_dict,
        }
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|(_, rw)| {
                let x = rw.read().unwrap();
                (x.get_task_name(), x.get_params())
            })
            .collect()
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    /// assigns task values (Python variables in which they will be stored)
    /// to each member of the code block.
    fn compute_task_vals(
        constraints: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: &String,
        tasks_dict: &Option<AST>,
    ) -> Vec<(AST, Arc<RwLock<ConstraintState<'a>>>)> {
        let mut out = Vec::new();
        for rw in constraints.into_iter() {
            let read = rw.read().unwrap();
            let name = read.get_task_name();
            drop(read);
            // TODO: magic number
            let task_val = match tasks_dict {
                None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name)),
                Some(ref dict) => {
                    let shorter_name =
                        name.replace(&format!("{}__", constraint_name).to_string(), "");

                    AST::Subscript(Subscript::new_wrapped(
                        dict.clone(),
                        AST::StringLiteral(StringLiteral::new_wrapped(shorter_name, false)),
                        false,
                    ))
                }
            };
            out.push((task_val, rw));
        }
        out
    }
    pub fn get_statements(
        &'a self,
        endpoints: &EndpointConfig,
        identifiers: &HashMap<Uuid, AST>,
    ) -> (Vec<AST>, LinkedHashSet<Preamble>, BTreeSet<Import>) {
        let tasks = self
            .members
            .iter()
            .map(|(task_val, rw)| {
                let x = rw.read().unwrap();
                let dep_uuids = x.get_dependencies();
                let dependencies = dep_uuids
                    .iter()
                    .map(|x| identifiers.get(x).unwrap().clone())
                    .collect();
                StandaloneETLTask::new(
                    x.get_task_name(),
                    task_val.clone(),
                    x.get_call(),
                    x.get_params(),
                    dependencies,
                    x.get_preamble(),
                    x.get_dialect(),
                )
            })
            .collect::<Vec<_>>();

        let mut compressible: LinkedHashMap<_, Vec<_>> = LinkedHashMap::new();
        let mut etl_tasks: Vec<ETLTask<T>> = Vec::new();

        for task in tasks.into_iter() {
            if task.is_compressible() {
                let key = task.get_compression_key().unwrap();
                compressible.entry(key).or_insert(Vec::new()).push(task);
            } else {
                etl_tasks.push(ETLTask::StandaloneETLTask(task));
            }
        }
        let mut num_compression_tasks = 0;
        for (compression_key, tasks) in compressible.into_iter() {
            // TODO: this is a magic number
            if tasks.len() > 2 {
                let params_constraint = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    match num_compression_tasks {
                        0 => format!("params_{}", self.get_constraint_name()).to_string(),
                        _ => format!(
                            "params_{}_{}",
                            self.get_constraint_name(),
                            num_compression_tasks + 1
                        )
                        .to_string(),
                    },
                ));
                let compressed_task = ForLoopETLTask::new(
                    params_constraint,
                    compression_key,
                    tasks
                        .into_iter()
                        .map(|x| x.get_uncompressible_part().unwrap())
                        .collect(),
                );
                etl_tasks.push(ETLTask::ForLoopETLTask(compressed_task));
                num_compression_tasks += 1;
            } else {
                for task in tasks.into_iter() {
                    etl_tasks.push(ETLTask::StandaloneETLTask(task));
                }
            }
        }
        let preambles_and_statements = etl_tasks
            .into_iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .map(|x| Preamble::new(x, py))
            .collect::<LinkedHashSet<Preamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        (
            preambles_and_statements
                .iter()
                .map(|x| x.0.clone())
                .flatten()
                .collect::<Vec<_>>(),
            preambles,
            imports,
        )
    }
}
