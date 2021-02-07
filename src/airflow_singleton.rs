use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    AoristStatement, ArgType, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Formatted,
    Import, List, PythonNone, SimpleIdentifier, StringLiteral,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use num_bigint::{BigInt, Sign};
use rustpython_parser::ast::{Location, Statement};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AirflowSingleton {
    task_id: ArgType,
    task_val: ArgType,
    command: Option<String>,
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
    dep_list: Option<ArgType>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
}
impl ETLSingleton for AirflowSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) => vec![Import::FromImport(
                "airflow.operators.python_operator".to_string(),
                "PythonOperator".to_string(),
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![Import::FromImport(
                    "airflow.operators.bash_operator".to_string(),
                    "BashOperator".to_string(),
                )]
            }
            None => vec![Import::FromImport(
                "airflow.operators.dummy_operator".to_string(),
                "DummyOperator".to_string(),
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
    fn get_task_val(&self) -> ArgType {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = ArgType::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AoristStatement::Assign(
            self.task_val.clone(),
            creation_expr,
        )];
        if let Some(ref dependencies) = self.dep_list {
            statements.push(AoristStatement::Expression(ArgType::Call(
                Call::new_wrapped(
                    ArgType::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".to_string(),
                        false,
                    )),
                    vec![dependencies.clone()],
                    LinkedHashMap::new(),
                ),
            )));
        }
        statements
    }
    fn new(
        task_id: ArgType,
        task_val: ArgType,
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
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
        }
    }
    fn compute_task_args(&self) -> Vec<ArgType> {
        Vec::new()
    }
    fn get_type() -> String {
        "airflow".to_string()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType> {
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
                Some(Dialect::Python(_)) => ArgType::SimpleIdentifier(
                    SimpleIdentifier::new_wrapped(self.command.as_ref().unwrap().clone()),
                ),
                Some(Dialect::Bash(_)) => ArgType::Formatted(Formatted::new_wrapped(
                    ArgType::StringLiteral(StringLiteral::new_wrapped(
                        self.command.as_ref().unwrap().clone(),
                    )),
                    self.kwargs.clone(),
                )),
                Some(Dialect::Presto(_)) => {
                    let mut fmt_args = LinkedHashMap::new();
                    fmt_args.insert(
                        "query".to_string(),
                        ArgType::Formatted(Formatted::new_wrapped(
                            ArgType::StringLiteral(StringLiteral::new_wrapped(
                                self.command.as_ref().unwrap().clone(),
                            )),
                            self.kwargs.clone(),
                        )),
                    );

                    ArgType::Formatted(Formatted::new_wrapped(
                        ArgType::StringLiteral(StringLiteral::new_wrapped(
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
                        ArgType::Dict(Dict::new_wrapped(self.kwargs.clone())),
                    );
                }
            }
        }
        kwargs.insert(
            "dag".to_string(),
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".to_string())),
        );
        kwargs.insert("task_id".to_string(), self.task_id.clone());
        kwargs
    }
    fn compute_task_call(&self) -> ArgType {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("PythonOperator".to_string()),
            )),
            Some(Dialect::Bash(_)) => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "BashOperator".to_string(),
            ))),
            Some(Dialect::Presto(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("BashOperator".to_string()),
            )),
            None => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "DummyOperator".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
}

pub struct AirflowDAG {}
impl ETLDAG for AirflowDAG {
    type T = AirflowSingleton;

    fn new() -> Self {
        Self {}
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow(&self, mut statements: Vec<Statement>, location: Location) -> Vec<Statement> {
        let default_args =
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("default_args".to_string()));
        let mut default_args_map: LinkedHashMap<String, ArgType> = LinkedHashMap::new();
        default_args_map.insert(
            "owner".to_string(),
            ArgType::StringLiteral(StringLiteral::new_wrapped("airflow".to_string())),
        );
        default_args_map.insert(
            "depends_on_past".to_string(),
            ArgType::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email".to_string(),
            ArgType::List(List::new_wrapped(
                vec![ArgType::StringLiteral(StringLiteral::new_wrapped(
                    "airflow@example.com".to_string(),
                ))],
                false,
            )),
        );
        default_args_map.insert(
            "email_on_failure".to_string(),
            ArgType::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email_on_retry".to_string(),
            ArgType::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "retries".to_string(),
            ArgType::BigIntLiteral(BigIntLiteral::new_wrapped(BigInt::new(Sign::Plus, vec![1]))),
        );
        default_args_map.insert(
            "retry_delay".to_string(),
            ArgType::BigIntLiteral(BigIntLiteral::new_wrapped(BigInt::new(
                Sign::Plus,
                vec![300],
            ))),
        );

        let default_args_dict = ArgType::Dict(Dict::new_wrapped(default_args_map));
        let default_args_assign = AoristStatement::Assign(default_args.clone(), default_args_dict);
        statements.insert(0, default_args_assign.statement(location));

        let dag = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".to_string()));

        let mut kwargs: LinkedHashMap<String, ArgType> = LinkedHashMap::new();
        kwargs.insert("default_args".to_string(), default_args);
        kwargs.insert(
            "description".to_string(),
            ArgType::StringLiteral(StringLiteral::new_wrapped(
                "Auto-generated by Aorist".to_string(),
            )),
        );
        kwargs.insert(
            "schedule_interval".to_string(),
            ArgType::PythonNone(PythonNone::new_wrapped()),
        );
        kwargs.insert(
            "start_date".to_string(),
            ArgType::Call(Call::new_wrapped(
                ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("datetime".to_string())),
                vec![
                    ArgType::BigIntLiteral(BigIntLiteral::new_wrapped(BigInt::new(
                        Sign::Plus,
                        vec![2021],
                    ))),
                    ArgType::BigIntLiteral(BigIntLiteral::new_wrapped(BigInt::new(
                        Sign::Plus,
                        vec![01],
                    ))),
                    ArgType::BigIntLiteral(BigIntLiteral::new_wrapped(BigInt::new(
                        Sign::Plus,
                        vec![01],
                    ))),
                ],
                LinkedHashMap::new(),
            )),
        );
        kwargs.insert(
            "tags".to_string(),
            ArgType::List(List::new_wrapped(
                vec![ArgType::StringLiteral(StringLiteral::new_wrapped(
                    "aorist".to_string(),
                ))],
                false,
            )),
        );
        let dag_call = ArgType::Call(Call::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("DAG".to_string())),
            vec![ArgType::StringLiteral(StringLiteral::new_wrapped(
                "flow".to_string(),
            ))],
            kwargs,
        ));
        let dag_call_assign = AoristStatement::Assign(dag, dag_call);
        statements.insert(1, dag_call_assign.statement(location));

        statements
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        vec![
            Import::FromImport("airflow".to_string(), "DAG".to_string()),
            Import::FromImport("datetime".to_string(), "datetime".to_string()),
        ]
    }
}
