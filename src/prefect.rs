#![allow(dead_code)]
use crate::constraint::{ArgType, LiteralsMap, StringLiteral};
use crate::constraint_state::{ConstraintState, PrefectSingleton};
use aorist_primitives::Dialect;
use indoc::formatdoc;
use rustpython_parser::ast::{
    Expression, ExpressionType, Keyword, Located, Location, Program, Statement, StatementType,
    StringGroup, Suite, WithItem,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, RwLock};

struct PrefectProgram {
    imports: Vec<Statement>,
    preamble: Vec<Statement>,
    flow: Vec<Statement>,
}
impl PrefectProgram {
    fn empty() -> Self {
        Self {
            imports: Vec::new(),
            preamble: Vec::new(),
            flow: Vec::new(),
        }
    }
    fn program(self) -> Program {
        let mut statements = self.imports;
        for elem in self.preamble {
            statements.push(elem);
        }
        let mut loc = statements.iter().last().unwrap().location;
        loc.newline();
        let with = StatementType::With {
            is_async: false,
            items: vec![WithItem {
                context_expr: Located {
                    location: loc,
                    node: ExpressionType::NamedExpression {
                        // TODO: change this to Flow(...)
                        left: Box::new(Located {
                            location: loc,
                            node: ExpressionType::Identifier {
                                name: "flow".into(),
                            },
                        }),
                        right: Box::new(Located {
                            location: loc,
                            node: ExpressionType::Identifier {
                                name: "flow".into(),
                            },
                        }),
                    },
                },
                optional_vars: None,
            }],
            body: self.flow,
        };
        statements.push(Located {
            location: loc,
            node: with,
        });
        Program { statements }
    }
    fn render_string_group(string_group: &StringGroup) -> Result<String, String> {
        match string_group {
            StringGroup::Constant { value } => match value.find('\n') {
                None => Ok(format!("'{}'", value).to_string()),
                Some(_) => Ok(format!("\"\"\"\n{}\"\"\"", value).to_string()),
            },
            StringGroup::FormattedValue {
                value,
                conversion,
                spec,
            } => match conversion {
                Some(_) => Err("Don't know how to apply conversion to string group".to_string()),
                None => match spec {
                    Some(s) => Ok(format!(
                        "{} % {}",
                        Self::render_string_group(s)?,
                        Self::render_expr(value)?,
                    )
                    .to_string()),
                    None => Err("Don't know what to do when spec missing.".to_string()),
                },
            },
            StringGroup::Joined { values } => Ok(format!(
                "({})",
                values
                    .iter()
                    .map(|x| Self::render_string_group(x).unwrap())
                    .collect::<Vec<String>>()
                    .join("\n")
            )),
        }
    }
    fn render_expr(expr: &Expression) -> Result<String, String> {
        match &expr.node {
            ExpressionType::String { value } => Self::render_string_group(value),
            ExpressionType::Subscript { a, b, .. } => {
                Ok(format!("{}[{}]", Self::render_expr(a)?, Self::render_expr(b)?).to_string())
            }
            ExpressionType::Identifier { name } => Ok(name.to_string()),
            ExpressionType::Attribute { value, name } => {
                Ok(format!("{}.{}", Self::render_expr(&value)?, name).to_string())
            }
            ExpressionType::Starred { value } => {
                Ok(format!("*{}", Self::render_expr(&value)?).to_string())
            }
            ExpressionType::Dict { elements } => Ok(format!(
                "{{{}}}",
                elements
                    .into_iter()
                    .map(|(k, v)| {
                        let key = match k {
                            None => "**".to_string(),
                            Some(expr) => Self::render_expr(expr).unwrap(),
                        };
                        let val = Self::render_expr(v).unwrap();
                        format!("{}: {}", key, val).to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(",\n")
            )),
            ExpressionType::List { elements } => Ok(format!(
                "[{}]",
                elements
                    .iter()
                    .map(|x| Self::render_expr(x).unwrap())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .to_string()),
            ExpressionType::Tuple { elements } => Ok(format!(
                "({})",
                elements
                    .iter()
                    .map(|x| Self::render_expr(x).unwrap())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
            .to_string()),
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                let mut formatted_args: Vec<String> = Vec::new();
                for arg in args {
                    formatted_args.push(Self::render_expr(&arg)?);
                }
                for keyword in keywords {
                    formatted_args.push(Self::render_keyword(&keyword)?);
                }
                Ok(format!(
                    "{}({})",
                    call = Self::render_expr(function)?,
                    args = formatted_args.join(", "),
                )
                .to_string())
            }
            _ => Err(format!("Unknown argument: {}", Expression::name(&expr))),
        }
    }
    fn render_keyword(keyword: &Keyword) -> Result<String, String> {
        match keyword.name {
            Some(ref name) => {
                Ok(format!("{}={}", name, Self::render_expr(&keyword.value)?).to_string())
            }
            None => Ok(format!("**{}", Self::render_expr(&keyword.value)?).to_string()),
        }
    }
    fn render_suite(suite: Suite) -> Result<String, String> {
        let mut rendered: Vec<String> = Vec::new();
        for stmt in suite {
            rendered.push(Self::render_statement(stmt)?);
        }
        Ok(rendered.join("\n"))
    }
    fn render_statement(statement: Statement) -> Result<String, String> {
        match statement.node {
            StatementType::Assign { targets, value, .. } => {
                if targets.len() != 1 {
                    return Err("More than one target in task assignment".to_string());
                }
                let val = Self::render_expr(targets.iter().next().unwrap())?;
                let call = Self::render_expr(&value)?;
                Ok(format!("{val} = {call}", val = val, call = call).to_string())
            }
            StatementType::Expression { expression } => Self::render_expr(&expression),
            StatementType::For {
                is_async,
                target,
                iter,
                body,
                orelse,
            } => {
                if is_async {
                    return Err("Cannot render async for statements".to_string());
                }
                if orelse.is_some() {
                    return Err("Cannot render for statements with orelse".to_string());
                }
                let body_fmt = body
                    .into_iter()
                    .map(|x| Self::render_statement(x).unwrap())
                    .map(|x| format!("    {}", x).to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(format!(
                    "for {target} in {iter}:\n{body_fmt}",
                    target = Self::render_expr(&target)?,
                    iter = Self::render_expr(&iter)?,
                    body_fmt = body_fmt
                )
                .to_string())
            }
            _ => Err("Unknown statement type.".to_string()),
        }
    }
}

pub trait PrefectTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>>;
    fn register_literals(
        &'a self,
        _literals: LiteralsMap,
        _constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
    }
    fn keywords_to_map(&self, keywords: Vec<Keyword>) -> HashMap<String, Expression> {
        assert!(keywords
            .iter()
            .filter(|x| x.name.is_none())
            .next()
            .is_none());
        keywords
            .into_iter()
            .map(|x| (x.name.unwrap(), x.value))
            .collect()
    }
    fn map_to_keywords(&self, map: HashMap<String, Expression>) -> Vec<Keyword> {
        map.into_iter()
            .map(|(k, v)| Keyword {
                name: Some(k),
                value: v,
            })
            .collect()
    }
    fn args_from_singletons(
        &self,
        input: HashMap<String, PrefectSingleton>,
        location: Location,
    ) -> HashMap<
        String,
        (
            Vec<Expression>,
            HashMap<String, Expression>,
            PrefectSingleton,
        ),
    > {
        let mut args_map: HashMap<
            String,
            (
                Vec<Expression>,
                HashMap<String, Expression>,
                PrefectSingleton,
            ),
        > = HashMap::new();
        for (task_name, mut singleton) in input.into_iter() {
            let args = Located {
                location,
                node: ExpressionType::Starred {
                    value: Box::new(Located {
                        location,
                        node: ExpressionType::Identifier {
                            name: "args".to_string(),
                        },
                    }),
                },
            };
            let kwargs = Keyword {
                name: None,
                value: Located {
                    location,
                    node: ExpressionType::Identifier {
                        name: "kwargs".to_string(),
                    },
                },
            };
            let (args_v, kwargs_v) = singleton.swap_params(vec![args], vec![kwargs]).unwrap();
            args_map.insert(
                task_name,
                (args_v, self.keywords_to_map(kwargs_v), singleton),
            );
        }
        args_map
    }
    fn get_singletons(
        &self,
        location: Location,
        literals: LiteralsMap,
    ) -> HashMap<String, PrefectSingleton> {
        let num_constraints = self.get_constraints().len();
        for rw in self.get_constraints() {
            let mut write = rw.write().unwrap();
            let name = write.get_task_name();
            // TODO: magic number
            if num_constraints <= 2 {
                write.set_task_val(ArgType::SimpleIdentifier(name));
            } else {
                write.set_task_val(ArgType::Subscript(
                    Box::new(ArgType::SimpleIdentifier("tasks".to_string())),
                    Box::new(ArgType::StringLiteral(Arc::new(RwLock::new(
                        StringLiteral::new(name),
                    )))),
                ));
            }
        }
        let singletons = self
            .get_constraints()
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                (
                    x.get_task_name(),
                    x.get_prefect_singleton(location, literals.clone()).unwrap(),
                )
            })
            .collect::<HashMap<String, _>>();
        singletons
    }
    fn extract_hashable_value_if_string_constant(&self, expr: &Expression) -> Option<String> {
        match expr.node {
            ExpressionType::String { ref value } => match value {
                StringGroup::Constant { ref value } => Some(value.clone()),
                _ => None,
            },
            _ => None,
        }
    }
    fn is_format_string_call(&self, expr: &Expression) -> bool {
        if let ExpressionType::Subscript { box ref b, .. } = &expr.node {
            if let ExpressionType::Identifier { ref name } = &b.node {
                return name.clone() == "format".to_string();
            }
        }
        return false;
    }
    fn extract_parameters_if_format_call(
        &self,
        expr: &Expression,
    ) -> Option<BTreeMap<String, String>> {
        match expr.node {
            ExpressionType::Call {
                box ref function,
                ref args,
                ref keywords,
            } => {
                if self.is_format_string_call(&function) {
                    assert!(args.len() == 0);
                    return Some(
                        keywords
                            .iter()
                            .map(|x| {
                                if let ExpressionType::String {
                                    value: StringGroup::Constant { value },
                                } = &x.value.node
                                {
                                    let key = x.name.as_ref().unwrap().clone();
                                    return (key, value.clone());
                                } else {
                                    panic!(".format() call with non-string args encountered.");
                                }
                            })
                            .collect(),
                    );
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }
    fn ident(&self, name: String, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::Identifier { name },
        }
    }
    fn literal(&self, value: String, location: Location) -> Expression {
        Located {
            location,
            node: ExpressionType::String {
                value: StringGroup::Constant { value },
            },
        }
    }
    fn compute_joint_args(
        &self,
        num_args: usize,
        args_names_values: &HashMap<usize, HashMap<String, Vec<String>>>,
        location: Location,
    ) -> Vec<Expression> {
        let mut joint_args: Vec<Result<Expression, String>> = Vec::new();
        for i in 0..num_args {
            joint_args.push(Err(format!("No solution found for arg. #{}", i).to_string()));
        }
        for (pos, v) in args_names_values.iter().filter(|(_, v)| v.len() == 1) {
            let param_unique_val = v.keys().next().unwrap();
            let arg = Ok(self.literal(param_unique_val.clone(), location));
            let prev = std::mem::replace(&mut joint_args[*pos], arg);
            assert!(!prev.is_ok());
        }
        joint_args.into_iter().map(|x| x.unwrap()).collect()
    }
    fn combine_params(
        &self,
        params_map: HashMap<
            String,
            (
                Vec<Expression>,
                HashMap<String, Expression>,
                PrefectSingleton,
            ),
        >,
        location: Location,
    ) -> (Suite, Vec<PrefectSingleton>) {
        let call_names: HashSet<String> = params_map
            .values()
            .map(|(_, _, x)| x.get_call_name().unwrap())
            .collect();
        assert_eq!(call_names.len(), 1);

        let param_names = params_map
            .values()
            .map(|(_, m, _)| m.keys().map(|x| x.clone()).collect::<HashSet<String>>());
        let mut param_name_hist: HashMap<String, usize> = HashMap::new();
        for name in param_names.flatten() {
            *param_name_hist.entry(name).or_insert(0) += 1;
        }
        // param_name => param_value => task_name
        let mut common_param_names_values: HashMap<String, HashMap<String, Vec<String>>> =
            HashMap::new();
        for (task_name, (_, kw, _)) in &params_map {
            for (param_name, expr) in kw.iter() {
                if let Some(val) = self.extract_hashable_value_if_string_constant(&expr) {
                    common_param_names_values
                        .entry(param_name.clone())
                        .or_insert(HashMap::new())
                        .entry(val)
                        .or_insert(Vec::new())
                        .push(task_name.clone());
                } else if let Some(params) = self.extract_parameters_if_format_call(&expr) {
                    // TODO: should double-check parameters don't occur twice
                    // w/ different values.
                    for (param_name, val) in params {
                        common_param_names_values
                            .entry(param_name.clone())
                            .or_insert(HashMap::new())
                            .entry(val)
                            .or_insert(Vec::new())
                            .push(task_name.clone());
                    }
                }
            }
        }
        let mut suite: Suite = Vec::new();
        for (param_name, v) in common_param_names_values
            .iter()
            .filter(|(_, v)| v.len() == 1)
        {
            let param_unique_val = v.keys().next().unwrap();
            let assign_stmt = Located {
                location,
                node: StatementType::Assign {
                    targets: vec![self.ident(param_name.clone(), location)],
                    value: self.literal(param_unique_val.clone(), location),
                },
            };
            suite.push(assign_stmt);
        }
        // arg_pos => arg_value => task_names
        let mut args_names_values: HashMap<usize, HashMap<String, Vec<String>>> = HashMap::new();

        let args_sizes: HashSet<usize> = params_map.values().map(|(x, _, _)| x.len()).collect();
        assert_eq!(args_sizes.len(), 1);
        let num_args = args_sizes.into_iter().next().unwrap();

        for (task_name, (args, _, _)) in &params_map {
            for (i, expr) in args.iter().enumerate() {
                if let Some(val) = self.extract_hashable_value_if_string_constant(&expr) {
                    args_names_values
                        .entry(i)
                        .or_insert(HashMap::new())
                        .entry(val)
                        .or_insert(Vec::new())
                        .push(task_name.clone());
                }
            }
        }
        let mut params: Vec<(Option<Expression>, Expression)> = Vec::new();
        let mut processed_singletons: Vec<PrefectSingleton> = Vec::new();

        for (task_name, (args_v, kwargs_v, mut singleton)) in params_map.into_iter() {
            let kws = self.map_to_keywords(kwargs_v);
            /*let kws: Vec<(Option<Expression>, Expression)> = kwargs_v
            .into_iter()
            .map(|(k, v)| {
                (
                    Some(Located {
                        location,
                        node: ExpressionType::String {
                            value: StringGroup::Constant { value: k },
                        },
                    }),
                    v,
                )
            })
            .collect();*/
            let task_name_ident = Located {
                location,
                node: ExpressionType::String {
                    value: StringGroup::Constant { value: task_name },
                },
            };
            let num_kws = kws.len();
            /*
            let kw_dict = Located {
                location,
                node: ExpressionType::Dict { elements: kws },
            };*/
            let arg_list = Located {
                location,
                node: ExpressionType::List { elements: args_v },
            };
            if num_kws > 0 && num_args > 0 {
                let tuple = Located {
                    location,
                    node: ExpressionType::Tuple {
                        elements: vec![arg_list], //, kw_dict],
                    },
                };
                params.push((Some(task_name_ident), tuple));
            } else if num_kws > 0 {
                //params.push((Some(task_name_ident), kw_dict));
            } else {
                params.push((Some(task_name_ident), arg_list));
            }
            let joint_args = self.compute_joint_args(num_args, &args_names_values, location);
            let (_args_v, _kwargs_v) = singleton.swap_params(joint_args, kws).unwrap();
            processed_singletons.push(singleton);
        }
        let params_dict = Located {
            location,
            node: ExpressionType::Dict { elements: params },
        };
        let params_stmt = Located {
            location,
            node: StatementType::Expression {
                expression: params_dict,
            },
        };
        suite.push(params_stmt);
        (suite, processed_singletons)
    }
    fn render(&self, location: Location, literals: LiteralsMap) {
        let singletons = self.get_singletons(location, literals);
        let params_map = self.args_from_singletons(singletons, location);
        let (params_suite, singletons) = self.combine_params(params_map, location);

        print!("{}\n", PrefectProgram::render_suite(params_suite).unwrap());
        for singleton in singletons {
            print!(
                "{}\n",
                PrefectProgram::render_suite(singleton.as_suite()).unwrap()
            );
        }
    }
    fn render_ids(ids: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> String {
        ids.iter()
            .map(|x| format!("'{}'", x.read().unwrap().get_task_name()).to_string())
            .collect::<Vec<String>>()
            .join(",\n    ")
    }
    fn render_dependencies(&self, constraint_name: String) -> Option<String> {
        if self
            .get_constraints()
            .iter()
            .map(|x| x.read().unwrap().get_satisfied_dependency_keys())
            .flatten()
            .next()
            .is_none()
        {
            return None;
        }
        Some(formatdoc!(
            "
        dependencies_{constraint} = {{
            {dependencies}
        }}
        ",
            constraint = constraint_name,
            dependencies = self
                .get_constraints()
                .iter()
                .map(|rw| {
                    let x = rw.read().unwrap();
                    formatdoc!(
                        "
               '{key}': [
                   {deps}
               ]",
                        key = x.get_task_name(),
                        deps = x
                            .get_satisfied_dependency_keys()
                            .iter()
                            .map(|x| format!("'{}'", x))
                            .collect::<Vec<_>>()
                            .join(",\n    "),
                    )
                    .to_string()
                    .replace("\n", "\n    ")
                })
                .collect::<Vec<_>>()
                .join(",\n    "),
        ))
    }
}
pub trait PrefectTaskRenderWithCalls<'a>: PrefectTaskRender<'a> {
    fn extract_call_for_rendering(rw: Arc<RwLock<ConstraintState<'a>>>) -> Option<String> {
        rw.read().unwrap().get_call()
    }
    fn render_single_call(&self, call_name: String, constraint_name: String);
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    );
}

pub struct PrefectPythonTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> PrefectTaskRender<'a> for PrefectPythonTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
}
impl<'a> PrefectTaskRenderWithCalls<'a> for PrefectPythonTaskRender<'a> {
    fn render_single_call(&self, call_name: String, constraint_name: String) {
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
            ### COMPARE
            {dependencies}
            for k, v in params_{constraint}.items():
                tasks[k] = {call}(*v)
                flow.add_node(tasks[k])
                for dep in dependencies_{constraint}[k]:
                    flow.add_edge(tasks[dep], tasks[k])
            ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
            for k, v in params_{constraint}.items():
                tasks[k] = {call}(*v)
                flow.add_node(tasks[k])
            ",
                    constraint = constraint_name,
                    call = call_name,
                )
            ),
        }
    }
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    ) {
        let ids = Self::render_ids(rws.clone());
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                    {dependencies}
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = {call}(*params_{constraint}[k])
                        flow.add_node(tasks[k])
                        for dep in dependencies_{constraint}[k]:
                            flow.add_edge(tasks[dep], tasks[k])
                    ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                    for k in [
                        {ids}
                    ]:
                        tasks[k] = {call}(*params_{constraint}[k])
                        flow.add_node(tasks[k])
                    ",
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
        }
    }
}
impl<'a> PrefectPythonTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { members }
    }
}
pub struct PrefectShellTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    dialect: Dialect,
}
impl<'a> PrefectTaskRender<'a> for PrefectShellTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn register_literals(&'a self, literals: LiteralsMap, state: Arc<RwLock<ConstraintState<'a>>>) {
        // TODO: this is super hacky, but it should do the job for now
        let read = state.read().unwrap();
        let call = match &self.dialect {
            Dialect::Presto(_) => {
                format!("presto -e '{}'", read.get_call().unwrap().clone()).to_string()
            }
            Dialect::Bash(_) => read.get_call().unwrap().clone(),
            _ => panic!("Unknown dialect encountered for PrefectShellTaskRender."),
        };
        let uuid = read.get_constraint_uuid();

        let mut write = literals.write().unwrap();
        let arc = write
            .entry(call.clone())
            .or_insert(Arc::new(RwLock::new(StringLiteral::new(call))));
        let mut arc_write = arc.write().unwrap();
        arc_write.register_object(uuid, Some("command".to_string()));
        drop(arc_write);
        drop(write);
    }
}
impl<'a> PrefectTaskRenderWithCalls<'a> for PrefectShellTaskRender<'a> {
    fn extract_call_for_rendering(rw: Arc<RwLock<ConstraintState<'a>>>) -> Option<String> {
        let maybe_call = rw.read().unwrap().get_call();
        match maybe_call {
            Some(call) => {
                let maybe_preamble = rw.read().unwrap().get_preamble();
                match maybe_preamble {
                    Some(preamble) => Some(format!("{}\n{}", preamble, call).to_string()),
                    None => Some(call),
                }
            }
            None => None,
        }
    }
    fn render_single_call(&self, call_name: String, constraint_name: String) {
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                        {dependencies}
                        for k, v in params_{constraint}.items():
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                {call} %s
                                \"\"\" % v.join(' '),
                            )
                            flow.add_node(tasks[k])
                            for dep in dependencies_{constraint}[k]:
                                flow.add_edge(tasks[dep], tasks[k])
                        ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name.replace("\n", "\n        "),
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                        for k, v in params_{constraint}.items():
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                {call} %s
                                \"\"\" % v.join(' '),
                            )
                            flow.add_node(tasks[k])
                        ",
                    constraint = constraint_name,
                    call = call_name.replace("\n", "\n        "),
                )
            ),
        }
    }
    fn render_multiple_calls(
        &self,
        call_name: String,
        constraint_name: String,
        rws: &Vec<Arc<RwLock<ConstraintState<'a>>>>,
    ) {
        let ids = Self::render_ids(rws.clone());
        match self.render_dependencies(constraint_name.clone()) {
            Some(dependencies) => println!(
                "{}",
                formatdoc!(
                    "
                        {dependencies}
                        for k in [{ids}]:
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                    {call} %s
                                \"\"\" % params_{constraint}[k].join(' '),
                            )
                            flow.add_node(tasks[k])
                            for dep in dependencies_{constraint}[k]:
                                flow.add_edge(tasks[dep], tasks[k])
                        ",
                    dependencies = dependencies,
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
            None => println!(
                "{}",
                formatdoc!(
                    "
                        for k in [{ids}]:
                            tasks[k] = ShellTask(
                                command=\"\"\"
                                    {call} %s
                                \"\"\" % params_{constraint}[k].join(' '),
                            )
                            flow.add_node(tasks[k])
                        ",
                    constraint = constraint_name,
                    call = call_name,
                    ids = ids,
                )
            ),
        }
    }
}
impl<'a> PrefectShellTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>, dialect: Dialect) -> Self {
        Self { members, dialect }
    }
}
pub struct PrefectConstantTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> PrefectConstantTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { members }
    }
}
impl<'a> PrefectTaskRender<'a> for PrefectConstantTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
}

pub enum PrefectRender<'a> {
    Python(PrefectPythonTaskRender<'a>),
    Shell(PrefectShellTaskRender<'a>),
    Constant(PrefectConstantTaskRender<'a>),
}
impl<'a> PrefectRender<'a> {
    pub fn render(&'a self, location: Location, literals: LiteralsMap) {
        match &self {
            PrefectRender::Python(x) => x.render(location, literals),
            PrefectRender::Shell(x) => x.render(location, literals),
            PrefectRender::Constant(x) => x.render(location, literals),
        }
    }
    pub fn register_literals(
        &'a self,
        literals: LiteralsMap,
        constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        //task_render.register_literals(literals.clone(), members.clone());
        match &self {
            PrefectRender::Python(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Shell(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Constant(x) => x.register_literals(literals, constraint_state),
        }
    }
}
