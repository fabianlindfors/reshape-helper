# Reshape Rust helper

This is a Rust helper library for the automated, zero-downtime schema migration tool [Reshape](https://github.com/fabianlindfors/reshape). To achieve zero-downtime migrations, Reshape requires that your application runs a simple query when it opens a connection to the database to select the right schema. This library automates that process and provides a macro to embed the right query directly in your application.

## Installation

Add `reshape_helper` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
reshape_helper = "0.1.0"
```

## Usage

The library exposes a single macro which will find all your Reshape migration files and determine the right schema query to run. The macro will be evaluated at compile time and embed directly in your binary, so you won't need to keep your migration files around at runtime.

The following is an example of how to use the library together with [SQLx](https://github.com/launchbadge/sqlx):

```rust
use reshape_helper::schema_query
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() {
	let reshape_schema_query = schema_query!();

	let pool = PgPoolOptions::new()
		.after_connect(|conn| Box::pin(async move {
			conn.execute(reshape_schema_query).await?;
			Ok(())
		}))
		.connect("postgres://postgres@localhost:5432/db").await?;
}
```

By default, `schema_query!` will look for migrations files in `migrations/` but you can specify your own directories as well:

```rust
use reshape_helper::schema_query

fn main() {
	let reshape_schema_query = schema_query!(
		"src/users/migrations",
		"src/todos/migrations",
	);

	// Execute reshape_schema_query against your database...
}
```

## License

Released under the [MIT license](https://choosealicense.com/licenses/mit/).