use serde::Serialize;
use serde_json::json;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Serialize)]
pub enum Payload {
    NOOP,
    SendEmail { email: String },
}

struct JobLight {
    id: i64,
}

struct Job {
    id: i64,
    // status: String,
    payload: Payload,
}

async fn must_get_pool() -> Pool<Postgres> {
    PgPool::connect("postgres://postgres:leak-ok-ieQu5ahh4P@localhost:5433/my_app")
        .await
        .expect("Could not connect to the database!")
}

fn insert_jobs_query() -> Query<'static, Postgres, PgArguments> {
    println!("Inserting jobs...");

    sqlx::query!(
        r#"
        INSERT INTO jobs (payload)
        VALUES ($1)
             , ($2)
             , ($3)
    "#,
        json!(Payload::NOOP),
        json!(Payload::SendEmail {
            email: "user1@example.com".to_string()
        }),
        json!(Payload::SendEmail {
            email: "user2@example.com".to_string()
        }),
    )
}

#[tokio::main]
async fn main() {
    let pg_pool = must_get_pool().await;

    insert_jobs_query()
        .execute(&pg_pool)
        .await
        .expect("Could not insert jobs");

    let jobs = sqlx::query_as!(
        Job,
        r#"
            UPDATE jobs
            SET status = 'Running'
            WHERE id IN (
                SELECT id
                FROM jobs
                WHERE status = 'Queued'
                ORDER BY id
                LIMIT 5
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, payload
            "#
    );
    /*
    ERROR HERE:
            Checking sqlx-pb v0.1.0 (/home/benjamin/code/explore/rust/sqlx-pb)
    error[E0308]: mismatched types
      --> src/main.rs:83:16
       |
    83 |       let jobs = sqlx::query_as!(
       |  ________________^
    84 | |         Job,
    85 | |         r#"
    86 | |             UPDATE jobs
    ...  |
    97 | |             "#
    98 | |     );
       | |_____^ expected enum `Payload`, found enum `serde_json::Value`
       |
       = note: this error originates in the macro `$crate::sqlx_macros::expand_query` (in Nightly builds, run with -Z macro-backtrace for more info)

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `sqlx-pb` due to previous error
    [Finished running. Exit status: 101]

         */

    ()
}
