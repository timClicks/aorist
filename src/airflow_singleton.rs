use crate::endpoints::EndpointConfig;
use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    Assignment, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Expression, Formatted,
    Import, List, PythonNone, SimpleIdentifier, StringLiteral, AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AirflowSingleton {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
    endpoints: EndpointConfig,
}
impl AirflowSingleton {
    fn compute_task_args(&self) -> Vec<AST> {
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        let mut kwargs;
        if self.dialect.is_none() {
            kwargs = self.kwargs.clone();
        } else {
            kwargs = LinkedHashMap::new();
            let call_param_name = match self.dialect {
                Some(Dialect::Python(_)) => "python_callable".to_string(),
                Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "bash_command".to_string(),
                _ => panic!("Dialect not supported"),
            };
            let call_param_value = match self.dialect {
                Some(Dialect::Python(_)) => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                Some(Dialect::Bash(_)) => AST::Formatted(Formatted::new_wrapped(
                    AST::StringLiteral(StringLiteral::new_wrapped(
                        self.command.as_ref().unwrap().clone(),
                    )),
                    self.kwargs.clone(),
                )),
                Some(Dialect::Presto(_)) => {
                    let mut fmt_args = LinkedHashMap::new();
                    fmt_args.insert(
                        "query".to_string(),
                        AST::Formatted(Formatted::new_wrapped(
                            AST::StringLiteral(StringLiteral::new_wrapped(
                                self.command.as_ref().unwrap().clone(),
                            )),
                            self.kwargs.clone(),
                        )),
                    );

                    AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(StringLiteral::new_wrapped(
                            "presto -e \"{query}\"".to_string(),
                        )),
                        fmt_args,
                    ))
                }
                _ => panic!("Dialect not supported"),
            };
            kwargs.insert(call_param_name, call_param_value);
            if let Some(Dialect::Python(_)) = self.dialect {
                if self.kwargs.len() > 0 {
                    kwargs.insert(
                        "op_kwargs".to_string(),
                        AST::Dict(Dict::new_wrapped(self.kwargs.clone())),
                    );
                }
            }
        }
        kwargs.insert(
            "dag".to_string(),
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".to_string())),
        );
        kwargs.insert("task_id".to_string(), self.task_id.clone());
        kwargs
    }
    fn compute_task_call(&self) -> AST {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "PythonOperator".to_string(),
            ))),
            Some(Dialect::Bash(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "BashOperator".to_string(),
            ))),
            Some(Dialect::Presto(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "BashOperator".to_string(),
            ))),
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "DummyOperator".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
}
impl ETLSingleton for AirflowSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) => vec![Import::FromImport(
                "airflow.operators.python_operator".to_string(),
                "PythonOperator".to_string(),
                None,
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![Import::FromImport(
                    "airflow.operators.bash_operator".to_string(),
                    "BashOperator".to_string(),
                    None,
                )]
            }
            None => vec![Import::FromImport(
                "airflow.operators.dummy_operator".to_string(),
                "DummyOperator".to_string(),
                None,
            )],
        }
    }
    fn get_preamble(&self) -> Vec<String> {
        let preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![p.clone()],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AST::Assignment(Assignment::new_wrapped(
            self.task_val.clone(),
            creation_expr,
        ))];
        if let Some(ref dependencies) = self.dep_list {
            statements.push(AST::Expression(Expression::new_wrapped(AST::Call(
                Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".to_string(),
                        false,
                    )),
                    vec![dependencies.clone()],
                    LinkedHashMap::new(),
                ),
            ))));
        }
        statements
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        endpoints: EndpointConfig,
    ) -> Self {
        Self {
            task_id,
            task_val,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
            endpoints,
        }
    }
    fn get_type() -> String {
        "airflow".to_string()
    }
}

pub struct AirflowDAG {}
impl ETLDAG for AirflowDAG {
    type T = AirflowSingleton;

    fn new() -> Self {
        Self {}
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        py: Python<'a>,
        mut statements: Vec<Vec<&'a PyAny>>,
        ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)> {
        let default_args =
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("default_args".to_string()));
        let mut default_args_map: LinkedHashMap<String, AST> = LinkedHashMap::new();
        default_args_map.insert(
            "owner".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped("airflow".to_string())),
        );
        default_args_map.insert(
            "depends_on_past".to_string(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email".to_string(),
            AST::List(List::new_wrapped(
                vec![AST::StringLiteral(StringLiteral::new_wrapped(
                    "airflow@example.com".to_string(),
                ))],
                false,
            )),
        );
        default_args_map.insert(
            "email_on_failure".to_string(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email_on_retry".to_string(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "retries".to_string(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
        );
        default_args_map.insert(
            "retry_delay".to_string(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(300)),
        );

        let default_args_dict = AST::Dict(Dict::new_wrapped(default_args_map));
        let default_args_assign = AST::Assignment(Assignment::new_wrapped(
            default_args.clone(),
            default_args_dict,
        ));

        let dag = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".to_string()));

        let mut kwargs: LinkedHashMap<String, AST> = LinkedHashMap::new();
        kwargs.insert("default_args".to_string(), default_args);
        kwargs.insert(
            "description".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped(
                "Auto-generated by Aorist".to_string(),
            )),
        );
        kwargs.insert(
            "schedule_interval".to_string(),
            AST::PythonNone(PythonNone::new_wrapped()),
        );
        kwargs.insert(
            "start_date".to_string(),
            AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("datetime".to_string())),
                vec![
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(2021)),
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
                ],
                LinkedHashMap::new(),
            )),
        );
        kwargs.insert(
            "tags".to_string(),
            AST::List(List::new_wrapped(
                vec![AST::StringLiteral(StringLiteral::new_wrapped(
                    "aorist".to_string(),
                ))],
                false,
            )),
        );
        let dag_call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("DAG".to_string())),
            vec![AST::StringLiteral(StringLiteral::new_wrapped(
                "flow".to_string(),
            ))],
            kwargs,
        ));
        let dag_call_assign = AST::Assignment(Assignment::new_wrapped(dag, dag_call));
        let dag_call_assign_ast = dag_call_assign.to_python_ast_node(py, ast_module).unwrap();
        statements.insert(
            0,
            vec![
                default_args_assign
                    .to_python_ast_node(py, ast_module)
                    .unwrap(),
                dag_call_assign_ast,
            ],
        );
        statements
            .into_iter()
            .enumerate()
            .map(|(i, x)| (format!("Block {}", i).to_string(), x))
            .collect()
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        vec![
            Import::FromImport("airflow".to_string(), "DAG".to_string(), None),
            Import::FromImport("datetime".to_string(), "datetime".to_string(), None),
        ]
    }
}
