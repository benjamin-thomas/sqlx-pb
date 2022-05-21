use sqlx::Row;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use serde_json::json;
use serde::Serialize;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "JOB_STATUS")]
enum JobStatus {
    Queued,
    Running,
    Failed,
}

#[derive(Serialize)]
enum Payload {
    NOOP,
    SendEmail { email: String },
}

#[derive(sqlx::FromRow)]
struct Job {
    id: i64,
    status: JobStatus,
}

async fn must_get_pool() -> Pool<Postgres> {
    PgPool::connect("postgres://postgres:leak-ok-123@localhost:5433/my_app")
        .await
        .expect("Could not connect to the database!")
}

fn insert_jobs() -> Query<'static, Postgres, PgArguments> {
    println!("Inserting jobs...");
    sqlx::query!(
        r#"
        INSERT INTO jobs (status, payload)
        VALUES ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
             , ($1, $2)
             , ($1, $3)
    "#,
        JobStatus::Queued as JobStatus,
        json!(Payload::NOOP),
        json!(Payload::SendEmail { email: "user@example.com".to_string() })
    )
}

#[tokio::main]
async fn main() {
    let pg_pool = must_get_pool().await;

    insert_jobs()
        .execute(&pg_pool)
        .await
        .expect("Could not insert");

    println!("1) ==> `query_as!`");
    println!("1) ==> Use SQL type override to fix this error: '{}'", r#"error: unsupported type job_status of column #2 ("status")"#);
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
            RETURNING id, status as "status: JobStatus"
            "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for job in jobs {
        println!("1) Working on job #{} ({:?})", job.id, job.status)
    }

    println!();
    println!("2) ==> `query_as`");
    println!("2) ==> this requires the `sqlx::FromRow` trait AND specifying the containing variable type (`Vec<Job>`)");
    let jobs: Vec<Job> = sqlx::query_as(
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
            RETURNING id, status
            "#,
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for x in jobs {
        println!("2) Working on job #{} ({:?})", x.id, x.status)
    }

    println!();
    println!("3) ==> `query!`");
    println!("3) ==> this requires the `sqlx::FromRow` trait AND the SQL type override");
    let records = sqlx::query!(
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
            RETURNING id, status as "status: JobStatus"
            "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for record in records {
        println!("3) Working on job #{} ({:?})", record.id, record.status)
    }

    println!();
    println!("4) ==> `query`");
    println!("4) ==> No requirements (manual conversion)");
    let pg_rows = sqlx::query(
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
            RETURNING id, status
            "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab rows!");

    for row in pg_rows {
        let id: i64 = row.try_get("id").unwrap();
        let status: JobStatus = row.try_get("status").unwrap();
        println!("4) Working on job #{} ({:?})", id, status)
    }

    ()
}
