use crate::expectation::{Check, Expectation};

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Float(f64),
    Text(String),
    TextArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub sql: String,
    pub params: Vec<Param>,
}

#[must_use]
pub fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// `schema.table` is split on the first dot; each part is quoted separately.
#[must_use]
pub fn table_ref(table: &str) -> String {
    match table.split_once('.') {
        Some((schema, name)) => format!("{}.{}", quote_ident(schema), quote_ident(name)),
        None => quote_ident(table),
    }
}

#[must_use]
pub fn count_query(e: &Expectation, table: &str) -> Query {
    let mut params = Vec::new();
    let condition = unexpected_condition(e, &mut params);
    let total = match e.check() {
        Check::NotNull {} => "count(*)".to_string(),
        _ => format!("count({})", quote_ident(e.column())),
    };
    Query {
        sql: format!(
            "SELECT count(*) FILTER (WHERE {condition}) AS unexpected, {total} AS total FROM {}",
            from_clause(e, table)
        ),
        params,
    }
}

#[must_use]
pub fn sample_query(e: &Expectation, table: &str, limit: u32) -> Query {
    let mut params = Vec::new();
    let condition = unexpected_condition(e, &mut params);
    Query {
        sql: format!(
            "SELECT {}::text FROM {} WHERE {condition} LIMIT {limit}",
            quote_ident(e.column()),
            from_clause(e, table)
        ),
        params,
    }
}

fn from_clause(e: &Expectation, table: &str) -> String {
    let table = table_ref(table);
    match e.check() {
        Check::Unique {} => format!(
            "(SELECT *, count(*) OVER (PARTITION BY {}) AS gx_dup_count FROM {table}) AS gx",
            quote_ident(e.column())
        ),
        _ => table,
    }
}

fn unexpected_condition(e: &Expectation, params: &mut Vec<Param>) -> String {
    let c = quote_ident(e.column());
    match e.check() {
        Check::NotNull {} => format!("{c} IS NULL"),
        Check::Unique {} => format!("{c} IS NOT NULL AND gx_dup_count > 1"),
        Check::Between { min, max } => {
            let min = bind(params, Param::Float(*min));
            let max = bind(params, Param::Float(*max));
            format!("{c} IS NOT NULL AND NOT ({c} BETWEEN {min}::float8 AND {max}::float8)")
        }
        Check::InSet { values } => {
            let set = bind(params, Param::TextArray(values.clone()));
            format!("{c} IS NOT NULL AND NOT ({c}::text = ANY({set}::text[]))")
        }
        Check::MatchRegex { pattern } => {
            let pattern = bind(params, Param::Text(pattern.clone()));
            format!("{c} IS NOT NULL AND NOT ({c}::text ~ {pattern})")
        }
    }
}

fn bind(params: &mut Vec<Param>, param: Param) -> String {
    params.push(param);
    format!("${}", params.len())
}
