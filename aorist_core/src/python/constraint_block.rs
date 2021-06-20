use crate::code::CodeBlock;
use crate::constraint::{SatisfiableOuterConstraint};
use aorist_primitives::OuterConstraint;
use crate::constraint_block::ConstraintBlock;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::python::PythonBasedCodeBlock;
use crate::python::{Assignment, Dict, PythonFlowBuilderInput, PythonImport, PythonPreamble, AST};
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PythonBasedConstraintBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a : 'b,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<PythonBasedCodeBlock<'a, 'b, T, C>>,
    tasks_dict: Option<AST>,
    _lt: PhantomData<&'a ()>,
    _constraint: PhantomData<C>,
}
impl<'a, 'b, T, C> ConstraintBlock<'a, 'b, T, C> for PythonBasedConstraintBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a : 'b,
{
    type C = PythonBasedCodeBlock<'a, 'b, T, C>;
    type BuilderInputType = PythonFlowBuilderInput;

    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn get_constraint_title(&self) -> Option<String> {
        self.title.clone()
    }
    fn get_constraint_body(&self) -> Option<String> {
        self.body.clone()
    }
    fn get_code_blocks(&self) -> &Vec<Self::C>
    {
        &self.members
    }

    fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<PythonBasedCodeBlock<'a, 'b, T, C>>,
        tasks_dict: Option<AST>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
            tasks_dict,
            _lt: PhantomData,
            _constraint: PhantomData,
        }
    }
    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|x| x.get_identifiers().into_iter())
            .flatten()
            .collect()
    }

    fn get_task_val_assignments(&self) -> Vec<AST> {
        match &self.tasks_dict {
            Some(ref val) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))],
            None => vec![],
        }
    }
}

impl<'a, 'b, T, C> PythonBasedConstraintBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a : 'b,
{
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
}
