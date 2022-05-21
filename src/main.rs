use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::types::Json;
use sqlx::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "JOB_STATUS")]
enum JobStatus {
    Queued,
    Running,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
enum Payload {
    NOOP,
    SendEmail { email: String },
}

#[derive(sqlx::FromRow)]
struct JobRow {
    id: i64,
    status: JobStatus,
    payload: Json<Payload>,
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
        json!(Payload::SendEmail {
            email: "user@example.com".to_string()
        })
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
    println!(
        "1) ==> Use SQL type override to fix this error: '{}'",
        r#"error: unsupported type job_status of column #2 ("status")"#
    );
    let jobs = sqlx::query_as!(
        JobRow,
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
            RETURNING id, status as "status: JobStatus", payload AS "payload: Json<Payload>"
            "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for job in jobs {
        println!(
            "1) Working on job #{} ({:?}) -> {:?}",
            job.id, job.status, job.payload
        );

        work_on_payload(&job.payload.0);
    }

    println!();
    println!("2) ==> `query_as`");
    println!("2) ==> this requires the `sqlx::FromRow` trait AND specifying the containing variable type (`Vec<Job>`)");
    let jobs: Vec<JobRow> = sqlx::query_as(
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
            RETURNING id, status, payload
            "#,
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for job in jobs {
        println!(
            "2) Working on job #{} ({:?}) -> {:?}",
            job.id, job.status, job.payload
        );
        work_on_payload(&job.payload.0);
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
            RETURNING id, status AS "status: JobStatus", payload
            "#
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab jobs!");

    for record in records {
        println!(
            "3) Working on job #{} ({:?}) -> {:?}",
            record.id, record.status, record.payload
        );
        work_on_payload(&serde_json::from_value(record.payload).unwrap())
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
            RETURNING id, status, payload
            "#,
    )
    .fetch_all(&pg_pool)
    .await
    .expect("failed to grab rows!");

    for row in pg_rows {
        let id: i64 = row.try_get("id").unwrap();
        let status: JobStatus = row.try_get("status").unwrap();
        let payload: Json<Payload> = row.try_get("payload").unwrap();
        println!("4) Working on job #{} ({:?}) -> {:?}", id, status, payload);
        work_on_payload(&payload);
    }

    ()
}

fn work_on_payload(payload: &Payload) {
    match payload {
        Payload::NOOP => println!("   --- NOOP!"),
        Payload::SendEmail { email } => {
            println!("   --- EMAIL[{}]", email.to_ascii_uppercase());
        }
    }
}
