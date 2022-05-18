use sqlx::postgres::PgRow;
use sqlx::{
    postgres::PgArguments,
    query::{Map, Query},
    Error, PgPool, Pool, Postgres,
};

struct Job {
    id: i64,
    // status: String,
    // message: String,
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
        INSERT INTO jobs (message)
        VALUES ('{"a": 1}')
             , ('{"b": 2}')
             , ('{"c": 3}')
             , ('{"d": 4}')
             , ('{"e": 5}')
    "#
    )
}

fn next_jobs_query() -> Map<'static, Postgres, fn(PgRow) -> Result<Job, Error>, PgArguments> {
    sqlx::query_as!(
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
            RETURNING id
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

    let jobs: Vec<Job> = next_jobs_query()
        .fetch_all(&pg_pool)
        .await
        .expect("Could not get jobs batch");

    for j in jobs {
        println!("Will work on job #{}", j.id)
    }

    ()
}
