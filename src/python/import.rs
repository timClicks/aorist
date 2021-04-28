use crate::code::Import;
use crate::python::ast::{SimpleIdentifier, AST};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PythonImport {
    PythonModuleImport(String, Option<String>),
    PythonFromImport(String, String, Option<String>),
}
impl Import for PythonImport {}

impl PythonImport {
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
        depth: usize,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::PythonModuleImport(ref module, ref alias) => {
                let alias_list = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                    module.clone(),
                                ))
                                .to_python_ast_node(py, ast_module, depth)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module, depth)?,
                            ),
                        )?,
                        None => {
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(module.clone()))
                                .to_python_ast_node(py, ast_module, depth)?
                        }
                    }],
                );
                ast_module.call1("PythonImport", (alias_list,))
            }
            Self::PythonFromImport(ref module, ref name, ref alias) => {
                let alias = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                                    .to_python_ast_node(py, ast_module, depth)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module, depth)?,
                            ),
                        )?,
                        None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                            .to_python_ast_node(py, ast_module, depth)?,
                    }],
                );
                ast_module.call1("PythonImportFrom", (module, alias.as_ref(), 0))
            }
        }
    }
}
