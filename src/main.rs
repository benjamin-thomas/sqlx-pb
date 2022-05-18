use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let pg_pool = PgPool::connect("postgres://postgres:leak-ok-ieQu5ahh4P@localhost:5433/my_app")
        .await
        .expect("Could not connect to the database!");

    println!("Inserting jobs...");
    sqlx::query!(
        r#"
        INSERT INTO jobs (message)
        VALUES ('{"a": 1}')
             , ('{"b": 2}')
             , ('{"c": 3}')
             , ('{"d": 4}')
             , ('{"e": 5}')
    "#
    )
    .execute(&pg_pool)
    .await
    .expect("Could not insert job");

    println!(
        "Next step: '{}'",
        "INSERT INTO jobs (message) VALUES ('{}');"
    );
}
