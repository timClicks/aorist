use crate::attributes::Attribute;
use aorist_primitives::{TAttribute, TSQLAttribute};
use sqlparser::ast::{
    Expr, Ident, ObjectName, Query, Select, SelectItem, SetExpr, Statement, TableFactor,
    TableWithJoins,
};

pub trait SQLQuery {
    fn empty() -> Self;
    fn set_table_name(&mut self, n: String) -> Result<(), String>;
    fn set_columns(&mut self, attributes: &Vec<Attribute>) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct SQLCreateQuery {
    statement: Statement,
}

impl SQLQuery for SQLCreateQuery {
    fn empty() -> Self {
        let statement = Statement::CreateTable {
            external: false,
            if_not_exists: true,
            name: ObjectName(Vec::new()),
            columns: Vec::new(),
            constraints: Vec::new(),
            with_options: Vec::new(),
            file_format: None,
            location: None,
            query: None,
            without_rowid: true,
        };
        Self { statement }
    }
    fn set_table_name(&mut self, n: String) -> Result<(), String> {
        match &mut self.statement {
            Statement::CreateTable { name, .. } => {
                let ObjectName(table_vec) = name;
                assert!(table_vec.is_empty());
                table_vec.push(Ident::new(n));
                Ok(())
            }
            _ => Err("Inner value not a create table statement statement.".into()),
        }
    }
    fn set_columns(&mut self, attributes: &Vec<Attribute>) -> Result<(), String> {
        match &mut self.statement {
            // we set the same columns in the statement and the source
            Statement::CreateTable { columns, .. } => {
                assert!(columns.is_empty());
                for attribute in attributes.iter() {
                    columns.push(attribute.get_coldef());
                }
                Ok(())
            }
            _ => Err("Inner value not a create table statement.".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SQLInsertQuery {
    statement: Statement,
}
impl SQLQuery for SQLInsertQuery {
    fn empty() -> Self {
        //let projection = vec![SelectItem::Wildcard];
        let table = vec![TableWithJoins {
            relation: TableFactor::Table {
                // e.g.: vec![Ident::new("some_table")]
                name: ObjectName(Vec::new()),
                alias: None,
                args: Vec::new(),
                with_hints: Vec::new(),
            },
            joins: Vec::new(),
        }];
        let select = Select {
            distinct: false,
            top: None,
            projection: Vec::new(),
            from: table,
            selection: None,
            group_by: Vec::new(),
            having: None,
        };
        let query = Query {
            ctes: Vec::new(),
            body: SetExpr::Select(Box::new(select)),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            fetch: None,
        };
        let statement = Statement::Insert {
            table_name: ObjectName(Vec::new()),
            columns: Vec::new(),
            source: Box::new(query),
        };
        Self { statement }
    }
    fn set_table_name(&mut self, name: String) -> Result<(), String> {
        match &mut self.statement {
            Statement::Insert { table_name, .. } => {
                let ObjectName(table_vec) = table_name;
                assert!(table_vec.is_empty());
                table_vec.push(Ident::new(name));
                Ok(())
            }
            _ => Err("Inner value not an insert statement.".into()),
        }
    }
    fn set_columns(&mut self, attributes: &Vec<Attribute>) -> Result<(), String> {
        match &mut self.statement {
            // we set the same columns in the statement and the source
            Statement::Insert {
                columns,
                box source,
                ..
            } => {
                assert!(columns.is_empty());
                for attribute in attributes.iter() {
                    let column = Ident::new(attribute.get_name().clone());
                    columns.push(column);
                }
                match &mut source.body {
                    SetExpr::Select(box select) => {
                        // TODO: change this to column
                        for attribute in attributes.iter() {
                            let column = Ident::new(attribute.get_name().clone());
                            select
                                .projection
                                .push(SelectItem::UnnamedExpr(Expr::Identifier(column)));
                        }
                        Ok(())
                    }
                    _ => Err("source.body must be a Select".into()),
                }
            }
            _ => Err("Inner value not an insert statement.".into()),
        }
    }
}
