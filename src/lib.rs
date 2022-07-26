use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use darling::FromMeta;
use proc_macro::{Literal, TokenStream, TokenTree};
use serde::{Deserialize, Serialize};
use syn::{parse_macro_input, AttributeArgs};

const DEFAULT_SEARCH_PATH: &'static str = r#""$user", public"#;

#[proc_macro]
pub fn schema_query(args: TokenStream) -> TokenStream {
    let attribute_args = parse_macro_input!(args as AttributeArgs);
    let mut dirs: Vec<String> = attribute_args
        .iter()
        .map(|arg| String::from_nested_meta(arg).unwrap())
        .collect();

    if dirs.is_empty() {
        dirs.push("migrations".to_string());
    }

    let migrations = find_migrations(dirs);
    let latest_migration = migrations
        .last()
        .map(|name| format!("migration_{name}"))
        .unwrap_or(DEFAULT_SEARCH_PATH.to_string());
    let query = format!("SET search_path TO {latest_migration}");

    let literal = Literal::string(&query);
    TokenTree::Literal(literal).into()
}

fn find_migrations(dirs: Vec<String>) -> Vec<String> {
    let search_paths = dirs
        .iter()
        .map(Path::new)
        // Filter out all directories that don't exist
        .filter(|path| path.exists());

    // Find all files in the search paths
    let mut file_paths = Vec::new();
    for search_path in search_paths {
        let entries = fs::read_dir(search_path).unwrap();
        for entry in entries {
            let path = entry.unwrap().path();
            file_paths.push(path);
        }
    }

    // Sort all files by their file names (without extension)
    // The files are sorted naturally, e.g. "1_test_migration" < "10_test_migration"
    file_paths.sort_unstable_by(|path1, path2| {
        let file1 = path1.as_path().file_stem().unwrap().to_str().unwrap();
        let file2 = path2.as_path().file_stem().unwrap().to_str().unwrap();

        lexical_sort::natural_cmp(file1, file2)
    });

    file_paths
        .iter()
        .map(|path| {
            let mut file = File::open(path).unwrap();

            // Read file data
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            (path, data)
        })
        .map(|(path, data)| {
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap();
            let file_migration = decode_migration_file(&data, extension);

            let file_name = path.file_stem().and_then(|name| name.to_str()).unwrap();
            file_migration.name.unwrap_or_else(|| file_name.to_string())
        })
        .collect()
}

fn decode_migration_file(data: &str, extension: &str) -> FileMigration {
    match extension {
        "json" => serde_json::from_str(data).unwrap(),
        "toml" => toml::from_str(data).unwrap(),
        extension => panic!("unrecognized file extension '{}'", extension),
    }
}

#[derive(Serialize, Deserialize)]
struct FileMigration {
    name: Option<String>,
}
