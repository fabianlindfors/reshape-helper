use reshape_helper::schema_query;

#[test]
fn default_folder() {
    let query: &'static str = schema_query!();
    assert_eq!(query, "SET search_path TO migration_2_test_migration");
}

#[test]
fn custom_directory() {
    let query: &'static str = schema_query!("tests/fixtures/migrations-1");
    assert_eq!(query, "SET search_path TO migration_10_test_migration");
}

#[test]
fn multiple_directories() {
    let query: &'static str =
        schema_query!("tests/fixtures/migrations-1", "tests/fixtures/migrations-2");
    assert_eq!(query, "SET search_path TO migration_10_test_migration");
}

#[test]
fn custom_migration_name() {
    let query: &'static str = schema_query!("tests/fixtures/custom-migration-name");
    assert_eq!(query, "SET search_path TO migration_custom_migration_name");
}

#[test]
fn non_existent_directory() {
    let query: &'static str = schema_query!("tests/fixtures/non-existent");
    assert_eq!(query, r#"SET search_path TO "$user", public"#);
}
