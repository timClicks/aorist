#![allow(non_snake_case)]
use indoc::formatdoc;
use sqlparser::ast::{ColumnDef, DataType, Ident};

#[macro_export]
macro_rules! register_ast_nodes {
    ($name:ident, $($variant: ident,)+) => {

        #[derive(Clone)]
        pub enum $name {
            $(
                $variant(Arc<RwLock<$variant>>),
            )+
        }
        impl $name {
            pub fn register_object(&mut self, uuid: Uuid, tag: Option<String>) {
                match &self {
                    $(
                        Self::$variant(x) => x.write().unwrap().register_object(uuid,
                        tag),
                    )+
                }
            }
            pub fn get_descendants(&self) -> Vec<AST> {
                let mut queue = VecDeque::new();
                queue.push_back(self.clone());
                let mut current = queue.pop_front();
                let mut v: Vec<AST> = Vec::new();
                while let Some(elem) = current {
                    let direct_descendants = match &elem {
                        $(
                            Self::$variant(x) => {
                            let read = x.read().unwrap();
                            read.get_direct_descendants()
                            }
                        )+
                    };
                    for desc in direct_descendants.into_iter() {
                        queue.push_back(desc);
                    }
                    v.push(elem);
                    current = queue.pop_front();
                }
                v
            }
            pub fn name(&self) -> String {
                match &self {
                    $(
                        Self::$variant(..) => stringify!($variant),
                    )+
                }
                .to_string()
            }
            pub fn to_python_ast_node<'a>(
                &self,
                py: Python,
                ast_module: &'a PyModule,
            ) -> PyResult<&'a PyAny> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().to_python_ast_node(
                            py,
                            ast_module
                        ),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().unwrap().eq(&v2.read().unwrap())
                        },
                    )+
                    (_, _) => false,
                }
            }
        }
        impl Eq for $name {}
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().hash(state),
                    )+
                }
            }
        }
    }
}
#[macro_export]
macro_rules! gdpr_data_type {
    ($name:ident
     $(, $field: ident : $field_type: ty)*) => {
        #[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize, FromPyObject)]
        pub struct $name {
            description: String,
            $(
                $field: $field_type,
            )*
        }
     };
}

#[macro_export]
macro_rules! define_task_node {
    ($name:ident,
     $descendants:expr,
     $py_ast_closure:expr,
     $import_closure:expr,
     $($field: ident : $field_type: ty,)*) => {
        #[derive(Hash, PartialEq, Eq, Clone)]
        pub struct $name {
            $(
                $field: $field_type,
            )*
        }
        impl $name {
            pub fn new_wrapped($(
                $field: $field_type,
            )*) -> Arc<RwLock<Self>> {
                Arc::new(RwLock::new(Self::new($($field, )*)))
            }
            pub fn get_statements<'a>(
                &self,
            ) -> Vec<AST> {
                ($py_ast_closure)(self)
            }
            pub fn new($(
                $field: $field_type,
            )*) -> Self {
                Self {
                    $($field,)*
                }
            }
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field.clone()
                }
            )*
            pub fn get_direct_descendants(&self) -> Vec<AST> {
                $descendants(self)
            }
            pub fn get_imports(&self) -> Vec<Import> {
                $import_closure(self)
            }
        }
    };
}
#[macro_export]
macro_rules! register_task_nodes {
    ($name:ident, $($variant: ident,)+) => {

        #[derive(Clone)]
        pub enum $name {
            $(
                $variant(Arc<RwLock<$variant>>),
            )+
        }
        impl $name {
            pub fn get_imports(&self) -> Vec<Import>{
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().get_imports(),
                    )+
                }
            }
            pub fn get_statements(&self) -> Vec<AST> {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().get_statements(),
                    )+
                }
            }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                match (&self, other) {
                    $(
                        (Self::$variant(v1), Self::$variant(v2)) => {
                            v1.read().unwrap().eq(&v2.read().unwrap())
                        },
                    )+
                    (_, _) => false,
                }
            }
        }
        impl Eq for $name {}
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match &self {
                    $(
                        Self::$variant(x) => x.read().unwrap().hash(state),
                    )+
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_ast_node {
    ($name:ident,
     $descendants:expr,
     $py_ast_closure:expr,
     $($field: ident : $field_type: ty,)*) => {
        #[derive(Hash, PartialEq, Eq, Clone)]
        pub struct $name {
            $(
                $field: $field_type,
            )*
        }
        impl $name {
            pub fn new_wrapped($(
                $field: $field_type,
            )*) -> Arc<RwLock<Self>> {
                Arc::new(RwLock::new(Self::new($($field, )*)))
            }
            pub fn to_python_ast_node<'a>(
                &self,
                py: Python,
                ast_module: &'a PyModule,
            ) -> PyResult<&'a PyAny> {
                ($py_ast_closure)(self, py, ast_module)
            }
            pub fn register_object(&self, _uuid: Uuid, _tag: Option<String>) {
                panic!(format!(
                    "Register object mistakenly called for object of type {}",
                    stringify!(name),
                ).to_string());
            }
            fn new($(
                $field: $field_type,
            )*) -> Self {
                Self {
                    $($field,)*
                }
            }
            $(
                pub fn $field(&self) -> $field_type {
                    self.$field.clone()
                }
            )*
            pub fn get_direct_descendants(&self) -> Vec<AST> {
                $descendants(self)
            }
        }
    };
}
#[macro_export]
macro_rules! define_program {
    ($name:ident, $root:ident, $constraint:ident, $satisfy_type:ident,
     $dialect:ident,
     $preamble:expr, $call:expr, $tuple_call: expr) => {
        pub struct $name {}
        impl ConstraintSatisfactionBase for $name {
            type RootType = $root;
            type ConstraintType = $constraint;
        }
        impl<'a> $satisfy_type<'a> for $name {
            type Dialect = $dialect;
            fn compute_parameter_tuple(
                uuid: Uuid,
                c: Concept<'a>,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> ParameterTuple {
                $tuple_call(uuid, c, ancestry)
            }
            fn get_preamble() -> String {
                $preamble.to_string()
            }
            fn get_call() -> String {
                $call.to_string()
            }
        }
    };
}

#[macro_export]
macro_rules! register_programs_for_constraint {
    ($constraint:ident, $root: ident,
     $($dialect:ident, $element: ident),+) => {
        impl SatisfiableConstraint for $constraint {
            fn satisfy<'a>(
                &mut self,
                c: Concept<'a>,
                d: &Dialect,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> Option<(String, String, ParameterTuple)> {
                match d {
                    $(
                        Dialect::$dialect{..} => Some((
                            $element::get_preamble(),
                            $element::get_call(),
                            $element::compute_parameter_tuple(
                                self.get_uuid().clone(),
                                c.clone(),
                                ancestry,
                            ),
                        )),
                    )+
                    _ => None,
                }
            }
            fn satisfy_given_preference_ordering<'a>(
                &mut self,
                c: Concept<'a>,
                preferences: &Vec<Dialect>,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> Result<(String, String, ParameterTuple, Dialect), String> {
                match c {
                    Concept::$root{..} => {
                        for d in preferences {
                            if let Some(
                                (preamble, call, params)
                            ) = self.satisfy(
                                c.clone(),
                                &d,
                                ancestry.clone(),
                            ) {
                                return Ok((preamble, call, params, d.clone()));
                            }
                        }
                        Err("Cannot satisfy preference ordering.".into())
                    },
                    _ => Err(format!("Wrong type of concept provided: {}",
                    c.get_type()))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! register_satisfiable_constraints {

    ($($constraint:ident),+)  => {
        impl AllConstraintsSatisfiability for Constraint {
            fn satisfy_given_preference_ordering<'a> (
                &mut self,
                c: Concept<'a>,
                preferences: &Vec<Dialect>,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> Result<(String, String, ParameterTuple, Dialect), String> {
                match &mut self.inner {
                    $(
                        Some(AoristConstraint::$constraint(ref mut x)) =>
                        x.satisfy_given_preference_ordering(
                            c, preferences,
                            ancestry,
                        ),
                    )+
                    _ => Err(format!("Constraint {} is not satisfiable (no program provided).", self.inner.as_ref().unwrap().get_name()).to_string())
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_attribute {
    ($element:ident, $presto_type:ident, $orc_type:ident, $sql_type:ident) => { paste::item! {
        #[aorist_concept(derive($presto_type, $orc_type, $sql_type))]
        pub struct $element {
            pub name: String,
            #[py_default = "None"]
            pub comment: Option<String>,
        }
        impl TAttribute for $element {
            fn get_name(&self) -> &String {
                &self.name
            }
            fn get_comment(&self) -> &Option<String> {
                &self.comment
            }
        }
        #[pymethods]
        impl [<Inner $element>] {
            #[getter]
            pub fn name(&self) -> PyResult<String> {
                Ok(self.name.clone())
            }
        }
    }}
}

#[macro_export]
macro_rules! define_constraint {
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident,
    $title: expr, $body: expr, $should_add: expr) => {
        pub struct $element {
            id: Uuid,
            root_uuid: Uuid,
        }
        impl $element {
            pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                Vec::new()
            }
            pub fn get_downstream_constraints_ignore_chains(&self) -> Vec<Arc<RwLock<Constraint>>> {
                Vec::new()
            }
            pub fn _should_add<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                $should_add(root, ancestry)
            }
            pub fn get_uuid(&self) -> Uuid {
                self.id.clone()
            }
            pub fn get_root_uuid(&self) -> Uuid {
                self.root_uuid.clone()
            }
            pub fn requires_program(&self) -> bool {
                $requires_program
            }
            pub fn ingest_upstream_constraints(
                &self,
                _upstream_constraints: Vec<Arc<RwLock<Constraint>>>
            ) {}
            pub fn get_title() -> Option<String> {
                $title
            }
            pub fn get_body() -> Option<String> {
                $body
            }
        }
        pub trait $satisfy_type<'a> : ConstraintSatisfactionBase<ConstraintType=$element, RootType=$root> {
            type Dialect;

            // computes a parameter tuple as a string, e.g. to be called from
            // Python
            fn compute_parameter_tuple(
                uuid: Uuid,
                root: Concept<'a>,
                ancestry: Arc<ConceptAncestry<'a>>,
            ) -> ParameterTuple;
            fn get_preamble() -> String;
            fn get_call() -> String;
        }
        impl TConstraint for $element {
            type Root = $root;

            fn get_root_type_name() -> String {
                stringify!($root).into()
            }
            fn get_required_constraint_names() -> Vec<String> {
                Vec::new()
            }
            fn new(root_uuid: Uuid,
                       _potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Self {
                Self{ id: Uuid::new_v4(), root_uuid}
            }
            fn should_add<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                match &root {
                    Concept::$root(x) => Self::_should_add(root, ancestry),
                    _ => panic!("should_add called with unexpected concept."),
                }
            }
        }
		impl fmt::Debug for $element {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				f.debug_struct(stringify!($element))
				 .field("id", &self.id)
				 .finish()
			}
		}
    };
    ($element:ident, $requires_program:expr, $satisfy_type:ident, $root:ident,
    $title:expr, $body:expr, $should_add:expr, $($required:ident),+) => {
        paste::item! {
            pub struct $element {
                id: Uuid,
                root_uuid: Uuid,
                $([<$required:snake:lower>] : Vec<Arc<RwLock<Constraint>>>,)+
            }
            pub trait $satisfy_type<'a> : ConstraintSatisfactionBase<ConstraintType=$element, RootType=$root> {
                type Dialect;

                // computes a parameter tuple as a string, e.g. to be called from
                // Python
                fn compute_parameter_tuple(
                    uuid: Uuid,
                    root: Concept<'a>,
                    ancestry: Arc<ConceptAncestry<'a>>,
                ) -> ParameterTuple;
                fn get_preamble() -> String;
                fn get_call() -> String;
            }
            impl $element {
                pub fn get_uuid(&self) -> Uuid {
                    self.id.clone()
                }
                pub fn _should_add<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                    $should_add(root, ancestry)
                }
                pub fn get_root_uuid(&self) -> Uuid {
                    self.root_uuid.clone()
                }
                pub fn requires_program(&self) -> bool {
                    $requires_program
                }
                pub fn ingest_upstream_constraints(
                    &mut self,
                    upstream_constraints: Vec<Arc<RwLock<Constraint>>>
                ) {
                    for constraint in upstream_constraints {
                        $(
                            if let Some(AoristConstraint::$required(x)) =
                            &constraint.read().unwrap().inner
                            {
                                self.[<$required:snake:lower>].push(constraint.clone());
                            }
                        )+
                    }
                }
                // these are *all* downstream constraints
                pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                    let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                    $(
                        for arc in &self.[<$required:snake:lower>] {
                            downstream.push(arc.clone());
                        }
                    )+
                    downstream
                }
                pub fn get_downstream_constraints_ignore_chains(&self) -> Vec<Arc<RwLock<Constraint>>> {
                    let mut downstream: Vec<Arc<RwLock<Constraint>>> = Vec::new();
                    $(
                        for arc in &self.[<$required:snake:lower>] {
                            downstream.push(arc.clone());
                        }
                    )+
                    loop {
                        if downstream.len() != 1 {
                            break;
                        }

                        let child = downstream.pop().unwrap();
                        if child.read().unwrap().requires_program() {
                            downstream.push(child);
                            break;
                        }

                        for elem in child.read().unwrap().get_downstream_constraints_ignore_chains() {
                            downstream.push(elem.clone());
                        }
                    }
                    downstream
                }
                pub fn get_title() -> Option<String> {
                    $title
                }
                pub fn get_body() -> Option<String> {
                    $body
                }
            }
            impl TConstraint for $element {
                type Root = $root;
                fn get_root_type_name() -> String {
                    stringify!($root).into()
                }
                fn get_required_constraint_names() -> Vec<String> {
                    vec![$(
                        stringify!($required).into()
                    ),+]
                }
                fn should_add<'a>(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool {
                    match &root {
                        Concept::$root(x) => Self::_should_add(root, ancestry),
                        _ => panic!("should_add called with unexpected concept."),
                    }
                }
                fn new(root_uuid: Uuid,
                       potential_child_constraints: Vec<Arc<RwLock<Constraint>>>) -> Self {
                    // TODO: we should dedupe potential child constraints
                    $(
                        let mut [<$required:snake:lower>]: Vec<Arc<RwLock<Constraint>>> =
                        Vec::new();
                    )+
                    let mut actual_child_constraints: Vec<Arc<RwLock<Constraint>>> = Vec::new();

                    for constraint in &potential_child_constraints {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner
                            {
                                actual_child_constraints.push(constraint.clone());
                            }
                        )+
                    }
                    let by_uuid: HashMap<Uuid, Arc<RwLock<Constraint>>> =
                    actual_child_constraints
                        .into_iter().map(|x| (x.clone().read().unwrap().get_uuid(), x)).collect();
                    for constraint in by_uuid.values() {
                        $(
                            if let Some(AoristConstraint::$required{..}) =
                            &constraint.read().unwrap().inner {
                                [<$required:snake:lower>].push(constraint.clone());
                            }
                        )+
                    }
                    Self{
                        id: Uuid::new_v4(),
                        root_uuid,
                        $([<$required:snake:lower>],)+
                    }
                }
            }
        }
    };
}
#[macro_export]
macro_rules! register_constraint {
    ( $name:ident, $($element: ident),+ ) => { paste::item! {
        pub enum $name {
            $(
                $element($element),
            )+
        }
        pub enum [<$name Builder>] {
            $(
                $element(crate::constraint::ConstraintBuilder<$element>),
            )+
        }
        impl [<$name Builder>] {
            pub fn get_root_type_name(&self) -> String {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn should_add<'a>(&self, root: Concept<'a>, ancestry:&ConceptAncestry<'a>) -> bool {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) =>
                        $element::should_add(root, ancestry),
                    )+
                }
            }
            pub fn build_constraint(
                &self,
                root_uuid: Uuid,
                potential_child_constraints: Vec<Arc<RwLock<Constraint>>>,
            ) -> Constraint {
                match &self {
                    $(
                        [<$name Builder>]::$element(x) => Constraint {
                            name: self.get_constraint_name(),
                            root: self.get_root_type_name(),
                            requires: Some(self.get_required_constraint_names()),
                            inner: Some(
                                $name::$element(x.build_constraint(
                                    root_uuid,
                                    potential_child_constraints,
                                ))
                            ),
                        },
                    )+
                }
            }
            pub fn get_required_constraint_names(&self) -> Vec<String> {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_constraint_name(&self) -> String {
                match &self {
                    $(
                        [<$name Builder>]::$element(_) => stringify!($element).to_string(),
                    )+
                }
            }
        }
        impl $name {
            pub fn builders() -> Vec<[<$name Builder>]> {
                vec![
                    $(
                        [<$name Builder>]::$element(
                            crate::constraint::ConstraintBuilder::<$element>{
                                _phantom: std::marker::PhantomData
                            }
                        ),
                    )+
                ]
            }
            pub fn get_root_type_name(&self) -> String {
                match self {
                    $(
                        Self::$element(_) => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
                match self {
                    $(
                        Self::$element(x) => x.get_downstream_constraints(),
                    )+
                }
            }
            pub fn get_downstream_constraints_ignore_chains(&self) -> Vec<Arc<RwLock<Constraint>>> {
                match self {
                    $(
                        Self::$element(x) =>
                        x.get_downstream_constraints_ignore_chains(),
                    )+
                }
            }
            pub fn requires_program(&self) -> bool {
                match self {
                    $(
                        Self::$element(x) => x.requires_program(),
                    )+
                }
            }
            pub fn ingest_upstream_constraints(
                &mut self,
                upstream_constraints: Vec<Arc<RwLock<Constraint>>>
            ) {
                match self {
                    $(
                        Self::$element(ref mut x) =>
                        x.ingest_upstream_constraints(upstream_constraints),
                    )+
                }
            }
            pub fn get_uuid(&self) -> Uuid {
                match self {
                    $(
                        Self::$element(x) => x.get_uuid(),
                    )+
                }
            }
            pub fn get_title(&self) -> Option<String> {
                match self {
                    $(
                        Self::$element(_) => $element::get_title(),
                    )+
                }
            }
            pub fn get_body(&self) -> Option<String> {
                match self {
                    $(
                        Self::$element(_) => $element::get_body(),
                    )+
                }
            }
            pub fn get_root_uuid(&self) -> Uuid {
                match self {
                    $(
                        Self::$element(x) => x.get_root_uuid(),
                    )+
                }
            }
            fn get_root_type_names() -> HashMap<String, String> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_root_type_name(),
                    )+
                }
            }
            pub fn get_required_constraint_names() -> HashMap<String, Vec<String>> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => $element::get_required_constraint_names(),
                    )+
                }
            }
            pub fn get_explanations() -> HashMap<String, (Option<String>,
            Option<String>)> {
                hashmap! {
                    $(
                        stringify!($element).to_string() => (
                            $element::get_title(),
                            $element::get_body(),
                        ),
                    )+
                }
            }
            pub fn get_name(&self) -> String {
                match self {
                    $(
                        Self::$element(x) => stringify!($element).to_string(),
                    )+
                }
            }
            pub fn should_add<'a>(&self, root: Concept<'a>, ancestry:
            &ConceptAncestry<'a>) -> bool {
                match &self {
                    $(
                        Self::$element(_) => $element::should_add(root,
                        ancestry),
                    )+
                }
            }
        }}
    }
}
#[macro_export]
macro_rules! register_attribute {
    ( $name:ident, $($element: ident),+ ) => { paste! {
        #[aorist_concept]
        pub enum $name {
            $(
                $element($element),
            )+
        }
        impl TAttribute for $name {
            fn get_name(&self) -> &String {
                match self {
                    $(
                        $name::$element(x) => x.get_name(),
                    )+
                }
            }
            fn get_comment(&self) -> &Option<String> {
                match self {
                    $(
                        $name::$element(x) => x.get_comment(),
                    )+
                }
            }
        }
        impl TAttribute for [<Inner $name>] {
            fn get_name(&self) -> &String {
                match self {
                    $(
                        [<Inner $name>]::$element(x) => &x.name,
                    )+
                }
            }
            fn get_comment(&self) -> &Option<String> {
                match self {
                    $(
                        [<Inner $name>]::$element(x) => &x.comment,
                    )+
                }
            }
        }
        impl TPrestoAttribute for $name {
            fn get_presto_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_presto_type(),
                    )+
                }
            }
        }
        impl TOrcAttribute for $name {
            fn get_orc_type(&self) -> String {
                match self {
                    $(
                        $name::$element(x) => x.get_orc_type(),
                    )+
                }
            }
        }
        impl TSQLAttribute for $name {
            fn get_sql_type(&self) -> DataType {
                match self {
                    $(
                        $name::$element(x) => x.get_sql_type(),
                    )+
                }
            }
        }
        paste::item!(
            pub fn [<$name:snake:lower>] (m: &PyModule) -> PyResult<()> {
                $(
                    m.add_class::<[<Inner $element>]>()?;
                )+
                Ok(())
            }
        );
    }}
}

pub trait TAttribute {
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &Option<String>;
}
pub trait TPrestoAttribute: TAttribute {
    fn get_presto_type(&self) -> String;
    fn get_presto_schema(&self, max_attribute_length: usize) -> String {
        let name = self.get_name();
        let num_middle_spaces = (max_attribute_length - name.len()) + 1;
        let spaces = (0..num_middle_spaces).map(|_| " ").collect::<String>();
        let first_line = format!("{}{}{}", self.get_name(), spaces, self.get_presto_type(),);
        if let Some(comment) = self.get_comment() {
            let formatted_with_comment = formatdoc!(
                "
                {first_line}
                     COMMENT '{comment}'",
                first_line = first_line,
                comment = comment.trim().replace("'", "\\'").to_string()
            );
            return formatted_with_comment;
        }
        first_line
    }
}
pub trait TOrcAttribute: TAttribute {
    fn get_orc_type(&self) -> String;
    fn get_orc_schema(&self) -> String {
        format!("{}:{}", self.get_name(), self.get_orc_type()).to_string()
    }
}
pub trait TSQLAttribute: TAttribute {
    fn get_sql_type(&self) -> DataType;
    fn get_coldef(&self) -> ColumnDef {
        ColumnDef {
            name: Ident::new(self.get_name()),
            data_type: self.get_sql_type(),
            collation: None,
            // TODO: add comments here
            options: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Python {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct R {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bash {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Presto {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
    Presto(Presto),
}

pub trait DownloadDataFromRemote {
    // TODO: change this to proper error
    fn get_call(&self, dialect: Dialect) -> Result<String, String>;
}

#[macro_export]
macro_rules! register_concept {
    ( $name:ident, $($element: ident ),* ) => { paste::item! {
        #[derive(Clone)]
        pub enum $name<'a> {
            $(
                $element((&'a $element, usize, Option<(Uuid, String)>)),
            )+
        }
        $(
            impl <'a> TryFrom<$name<'a>> for &'a $element {
                type Error = String;
                fn try_from(x: $name<'a>) -> Result<Self, String> {
                    match x {
                        $name::$element((y, _, _)) => Ok(y),
                        _ => Err("Cannot convert.".into()),
                    }
                }
            }
            impl <'a> TryFrom<&'a $name<'a>> for &'a $element {
                type Error = String;
                fn try_from(x: &'a $name<'a>) -> Result<Self, String> {
                    match x {
                        &$name::$element((y, _, _)) => Ok(y),
                        _ => Err("Cannot convert.".into()),
                    }
                }
            }
        )+
        pub struct ConceptAncestry<'a> {
            pub parents: Arc<RwLock<HashMap<(Uuid, String), $name<'a>>>>,
        }
        impl <'a> ConceptAncestry<'a> {
            $(
                pub fn [<$element:snake:lower>](
                    &self,
                    root: $name<'a>,
                ) -> Result<&'a $element, String> {
                    if root.get_type() == stringify!($element).to_string(){
                        return(Ok(<&'a $element>::try_from(root).unwrap()));
                    }
                    let parent_id = root.get_parent_id();
                    match parent_id {
                        None => Err(
                            format!(
                                "Cannot find ancestor of type {}.",
                                stringify!($element)
                            )
                        ),
                        Some(id) => {
                            let guard = self.parents.read().unwrap();
                            let parent = guard.get(&id).unwrap().clone();
                            self.[<$element:snake:lower>](parent)
                        }
                    }
                }
            )+
        }
        impl <'a> $name<'a> {
            pub fn get_parent_id(&'a self) -> Option<(Uuid, String)> {
                match self {
                    $(
                        $name::$element((_, _, id)) => id.clone(),
                    )+
                }
            }
            pub fn get_type(&'a self) -> String {
                match self {
                    $(
                        $name::$element((x, _, _)) => stringify!($element).to_string(),
                    )*
                }
            }
            pub fn get_uuid(&'a self) -> Uuid {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_uuid(),
                    )*
                }
            }
            pub fn get_tag(&'a self) -> Option<String> {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_tag(),
                    )*
                }
            }
            pub fn get_index_as_child(&'a self) -> usize {
                match self {
                    $(
                        $name::$element((_, idx, _)) => *idx,
                    )*
                }
            }
            pub fn get_child_concepts<'b>(&'a self) -> Vec<$name<'b>> where 'a : 'b {
                match self {
                    $(
                        $name::$element((x, _, _)) => x.get_child_concepts(),
                    )*
                }
            }
            pub fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Concept<'a>>) {
                match self {
                    $(
                        $name::$element((ref x, idx, parent)) => {
                            for child in x.get_child_concepts() {
                                child.populate_child_concept_map(concept_map);
                            }
                            concept_map.insert(
                                (x.get_uuid(),
                                 stringify!($element).to_string()),
                                 $name::$element((&x, *idx, parent.clone())),
                            );
                        }
                    )*
                }
            }
        }
    }
    }
}
