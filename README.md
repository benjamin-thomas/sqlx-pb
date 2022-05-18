# sqlx + JSON 

Trying to narrow down a type conversion problem.

## Setup

1. Clone this project
2. Run the following steps

```bash
# Create and start the dev database
docker-compose up db

# Migrate the database
cargo install sqlx-cli # requirement
DATABASE_URL=postgres://postgres:leak-ok-ieQu5ahh4P@localhost:5433/my_app sqlx migrate run

# Now run the app
DATABASE_URL=postgres://postgres:leak-ok-ieQu5ahh4P@localhost:5433/my_app cargo run
```

## Error
The error nagging me is here is documented here: `src/main.rs:77`

```
expected enum `Payload`, found enum `serde_json::Value`
```

To convert **to** JSON, I understood I need to:

- activate `sqlx`'s `json` feature
- use the `serde_json` crate to convert any data type with the `json!` macro
- use the `serde` crate as the process above requires `derive(Serialize)`

I fail to understand why I'm getting a `serde_json::Value` from the `sqlx::query_as!` macro.

And I fail to understand how to convert this `Value` type to my final type (I want the `jobs` variable's type to be `Vec<Job>`)