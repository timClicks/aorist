use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::python::{
    AoristStatement, Attribute, BigIntLiteral, Call, Dict, Import, List, ParameterTuple,
    ParameterTupleDedupKey, SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct StandaloneETLTask<T>
where
    T: ETLSingleton,
{
    /// where the task creation call should be stored.
    task_val: AST,
    /// unique task identifier
    task_id: String,
    /// function called to create task (has different meaning depending on
    /// the render we use.
    call: Option<String>,
    /// arguments passed to function call
    params: Option<ParameterTuple>,
    /// task_vals (or references to them) of other tasks this one
    /// depends on.
    dependencies: Vec<AST>,
    /// Python preamble used by this task call
    preamble: Option<String>,
    /// Dialect (e.g. Bash, Python, R, Presto, etc.), to be interpreted
    /// by render.
    dialect: Option<Dialect>,
    singleton_type: PhantomData<T>,
}
/// tuple of:
/// - name of dict / list in which task_val is stored (must be dict or list)
/// - function call (if any)
/// - from parameters:
///   - number of args
///   - names of kwargs
/// - preamble
/// - dialect
pub type ETLTaskCompressionKey = (
    // dict name
    AST,
    // function call
    Option<String>,
    // dedup key from parameters
    Option<ParameterTupleDedupKey>,
    // preamble
    Option<String>,
    // dialect
    Option<Dialect>,
);
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ETLTaskUncompressiblePart<T>
where
    T: ETLSingleton,
{
    // unique task_id
    task_id: String,
    // dict value
    pub dict: String,
    // params
    params: Option<ParameterTuple>,
    // dep list
    pub deps: Vec<AST>,
    singleton_type: PhantomData<T>,
}

impl<T> ETLTaskUncompressiblePart<T>
where
    T: ETLSingleton,
{
    pub fn new(
        task_id: String,
        dict: String,
        params: Option<ParameterTuple>,
        deps: Vec<AST>,
    ) -> Self {
        Self {
            task_id,
            dict,
            params,
            deps,
            singleton_type: PhantomData,
        }
    }

    pub fn as_python_dict(&self, dependencies_as_list: bool) -> AST {
        let mut local_params_map: LinkedHashMap<String, AST> = LinkedHashMap::new();
        if self.deps.len() > 0 {
            let dependencies = match dependencies_as_list {
                true => AST::List(List::new_wrapped(self.deps.clone(), false)),
                false => {
                    assert_eq!(self.deps.len(), 1);
                    self.deps.get(0).unwrap().clone()
                }
            };
            local_params_map.insert("dependencies".to_string(), dependencies);
        }
        // TODO: get_type should return an enum
        if T::get_type() == "airflow".to_string() {
            local_params_map.insert(
                "task_id".to_string(),
                AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone())),
            );
        }
        if let Some(ref p) = self.params {
            p.populate_python_dict(&mut local_params_map);
        }
        AST::Dict(Dict::new_wrapped(local_params_map))
    }
}

impl<T> StandaloneETLTask<T>
where
    T: ETLSingleton,
{
    /// only return true for compressible tasks, i.e. those that have a
    /// dict task val (in the future more stuff could be added here)
    pub fn is_compressible(&self) -> bool {
        match &self.task_val {
            AST::Subscript(_) => true,
            _ => false,
        }
    }
    fn get_left_of_task_val(&self) -> Result<AST, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                Ok(rw.a().clone())
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    fn get_right_of_task_val(&self) -> Result<String, String> {
        match &self.task_val {
            AST::Subscript(x) => {
                let rw = x.read().unwrap();
                match &rw.b() {
                    AST::StringLiteral(l) => Ok(l.read().unwrap().value().clone()),
                    _ => Err("Right of subscript must be a string
                    literal"
                        .to_string()),
                }
            }
            _ => Err("Task val must be a subscript".to_string()),
        }
    }
    pub fn get_compression_key(&self) -> Result<ETLTaskCompressionKey, String> {
        Ok((
            self.get_left_of_task_val()?,
            self.call.clone(),
            match &self.params {
                Some(p) => Some(p.get_dedup_key()),
                None => None,
            },
            self.preamble.clone(),
            self.dialect.clone(),
        ))
    }
    pub fn get_uncompressible_part(&self) -> Result<ETLTaskUncompressiblePart<T>, String> {
        Ok(ETLTaskUncompressiblePart::new(
            self.task_id.clone(),
            self.get_right_of_task_val()?,
            self.params.clone(),
            self.dependencies.clone(),
        ))
    }
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    pub fn new(
        task_id: String,
        task_val: AST,
        call: Option<String>,
        params: Option<ParameterTuple>,
        dependencies: Vec<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_id,
            task_val,
            call,
            params,
            dependencies,
            preamble,
            dialect,
            singleton_type: PhantomData,
        }
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AoristStatement>, Vec<String>, Vec<Import>) {
        let args;
        let kwargs;
        if let Some(ref p) = self.params {
            args = p.get_args();
            kwargs = p.get_kwargs();
        } else {
            args = Vec::new();
            kwargs = LinkedHashMap::new();
        }
        let singleton = T::new(
            AST::StringLiteral(StringLiteral::new_wrapped(self.task_id.clone())),
            self.get_task_val(),
            self.call.clone(),
            args,
            kwargs,
            match self.dependencies.len() {
                0 => None,
                _ => Some(AST::List(List::new_wrapped(
                    self.dependencies.clone(),
                    false,
                ))),
            },
            self.get_preamble(),
            self.get_dialect(),
        );
        (
            singleton.get_statements(endpoints),
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ForLoopETLTask<T>
where
    T: ETLSingleton,
{
    params_dict_name: AST,
    key: ETLTaskCompressionKey,
    values: Vec<ETLTaskUncompressiblePart<T>>,
    singleton_type: PhantomData<T>,
}
impl<T> ForLoopETLTask<T>
where
    T: ETLSingleton,
{
    pub fn new(
        params_dict_name: AST,
        key: ETLTaskCompressionKey,
        values: Vec<ETLTaskUncompressiblePart<T>>,
    ) -> Self {
        Self {
            params_dict_name,
            key,
            values,
            singleton_type: PhantomData,
        }
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AoristStatement>, Vec<String>, Vec<Import>) {
        let any_dependencies = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 0)
            .next()
            .is_some();
        let dependencies_as_list = self
            .values
            .iter()
            .filter(|x| x.deps.len() > 1)
            .next()
            .is_some();
        let dict_content = AST::Dict(Dict::new_wrapped(
            self.values
                .iter()
                .map(|x| (x.dict.clone(), x.as_python_dict(dependencies_as_list)))
                .collect(),
        ));
        let dict_assign = AoristStatement::Assign(self.params_dict_name.clone(), dict_content);

        let params = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".to_string()));
        let ident = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".to_string()));
        let tpl = AST::Tuple(Tuple::new_wrapped(
            vec![ident.clone(), params.clone()],
            false,
        ));
        let (dict, call, params_dedup_key, preamble, dialect) = self.key.clone();
        let new_collector =
            AST::Subscript(Subscript::new_wrapped(dict.clone(), ident.clone(), false));
        let kwargs;
        let args;
        if let Some((num_args, kwarg_keys)) = params_dedup_key {
            kwargs = kwarg_keys
                .iter()
                .map(|x| {
                    (
                        x.clone(),
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(x.to_string())),
                            false,
                        )),
                    )
                })
                .collect::<LinkedHashMap<_, _>>();

            args = (0..num_args)
                .map(|x| {
                    AST::Subscript(Subscript::new_wrapped(
                        AST::Subscript(Subscript::new_wrapped(
                            params.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped("args".to_string())),
                            false,
                        )),
                        AST::BigIntLiteral(BigIntLiteral::new_wrapped(x as i64)),
                        false,
                    ))
                })
                .collect::<Vec<AST>>();
        } else {
            kwargs = LinkedHashMap::new();
            args = Vec::new();
        }
        let dependencies = match any_dependencies {
            true => Some(AST::Subscript(Subscript::new_wrapped(
                params.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped("dependencies".to_string())),
                false,
            ))),
            false => None,
        };
        let task_id = AST::Subscript(Subscript::new_wrapped(
            params.clone(),
            AST::StringLiteral(StringLiteral::new_wrapped("task_id".to_string())),
            false,
        ));

        let singleton = T::new(
            task_id,
            new_collector.clone(),
            call,
            args,
            kwargs,
            dependencies,
            preamble.clone(),
            dialect.clone(),
        );
        let statements = singleton.get_statements(endpoints);
        let items_call = AST::Call(Call::new_wrapped(
            AST::Attribute(Attribute::new_wrapped(
                self.params_dict_name.clone(),
                "items".to_string(),
                false,
            )),
            Vec::new(),
            LinkedHashMap::new(),
        ));
        let for_loop = AoristStatement::For(tpl.clone(), items_call, statements.clone());
        (
            vec![dict_assign, for_loop],
            singleton.get_preamble(),
            singleton.get_imports(),
        )
    }
}

pub enum ETLTask<T>
where
    T: ETLSingleton,
{
    StandaloneETLTask(StandaloneETLTask<T>),
    ForLoopETLTask(ForLoopETLTask<T>),
}
impl<T> ETLTask<T>
where
    T: ETLSingleton,
{
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AoristStatement>, Vec<String>, Vec<Import>) {
        match &self {
            ETLTask::StandaloneETLTask(x) => x.get_statements(endpoints),
            ETLTask::ForLoopETLTask(x) => x.get_statements(endpoints),
        }
    }
}
