use sqlx::{postgres::PgArguments, query::Query, PgPool, Pool, Postgres};

async fn must_get_pool() -> Pool<Postgres> {
    PgPool::connect("postgres://postgres:leak-ok-ieQu5ahh4P@localhost:5433/my_app")
        .await
        .expect("Could not connect to the database!")
}

fn insert_jobs_query() -> Query<'static, Postgres, PgArguments> {
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
}

#[tokio::main]
async fn main() {
    let pg_pool = must_get_pool().await;

    insert_jobs_query()
        .execute(&pg_pool)
        .await
        .expect("Could not insert jobs");

}
