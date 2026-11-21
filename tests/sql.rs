use zuwaigani::backend::postgres::sql::{Param, count_query, quote_ident, sample_query, table_ref};
use zuwaigani::expectation::Expectation;

fn parse(yaml: &str) -> Expectation {
    serde_yaml::from_str(&format!("{yaml}\nplatform: [postgres]")).unwrap()
}

#[test]
fn quotes_identifiers() {
    assert_eq!(quote_ident("user_id"), r#""user_id""#);
    assert_eq!(
        quote_ident(r#"a"; DROP TABLE x; --"#),
        r#""a""; DROP TABLE x; --""#
    );
}

#[test]
fn splits_schema_from_table() {
    assert_eq!(table_ref("users"), r#""users""#);
    assert_eq!(table_ref("public.users"), r#""public"."users""#);
}

#[test]
fn not_null_counts_all_rows() {
    let q = count_query(&parse("type: not_null\ncolumn: id"), "users");
    assert_eq!(
        q.sql,
        r#"SELECT count(*) FILTER (WHERE "id" IS NULL) AS unexpected, count(*) AS total FROM "users""#
    );
    assert_eq!(q.params, Vec::new());
}

#[test]
fn unique_uses_a_window_over_the_column() {
    let q = count_query(&parse("type: unique\ncolumn: id"), "users");
    assert!(
        q.sql.contains(r#"count(*) OVER (PARTITION BY "id")"#),
        "{}",
        q.sql
    );
    assert!(q.sql.contains(r#"count("id") AS total"#), "{}", q.sql);
}

#[test]
fn values_are_bound_not_inlined() {
    let between = count_query(
        &parse("type: between\ncolumn: age\nmin: 0\nmax: 120"),
        "users",
    );
    assert_eq!(between.params, vec![Param::Float(0.0), Param::Float(120.0)]);
    assert!(
        between.sql.contains("BETWEEN $1::float8 AND $2::float8"),
        "{}",
        between.sql
    );

    let in_set = count_query(
        &parse("type: in_set\ncolumn: s\nvalues: [\"a'b\"]"),
        "users",
    );
    assert_eq!(in_set.params, vec![Param::TextArray(vec!["a'b".into()])]);
    assert!(!in_set.sql.contains("a'b"), "{}", in_set.sql);

    let regex = count_query(
        &parse("type: match_regex\ncolumn: e\npattern: '^x$'"),
        "users",
    );
    assert_eq!(regex.params, vec![Param::Text("^x$".into())]);
}

#[test]
fn sample_query_reuses_the_condition() {
    let q = sample_query(
        &parse("type: between\ncolumn: age\nmin: 0\nmax: 120"),
        "users",
        5,
    );
    assert!(
        q.sql
            .starts_with(r#"SELECT "age"::text FROM "users" WHERE"#),
        "{}",
        q.sql
    );
    assert!(q.sql.ends_with("LIMIT 5"), "{}", q.sql);
    assert_eq!(q.params.len(), 2);
}
