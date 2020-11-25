use sqlparser::ast::{
    Ident, ObjectName, Query, Select, SelectItem, SetExpr, Statement, TableFactor, TableWithJoins, Expr,
};
use crate::attributes::{Attribute, TAttribute};

#[derive(Debug, Clone)]
pub struct PrestoInsertQuery {
    statement: Statement,
}
impl PrestoInsertQuery {
    pub fn empty() -> Self {
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
    pub fn set_table_name(&mut self, name: String) -> Result<(), String> {
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
    pub fn set_columns(&mut self, attributes: &Vec<Attribute>) -> Result<(), String> {
        match &mut self.statement {
            // we set the same columns in the statement and the source
            Statement::Insert { columns, box source, .. } => {
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
                            select.projection.push(SelectItem::UnnamedExpr(Expr::Identifier(column)));
                        }
                        Ok(())
                    }
                    _ => Err("source.body must be a Select".into())
                }
                /*match source {
                    Query{ body, .. } => {
                        Ok(())
                    },
                    _ => Err("Statement source must be a Query."),
                }?;*/
            }
            _ => Err("Inner value not an insert statement.".into()),
        }
    }
}
