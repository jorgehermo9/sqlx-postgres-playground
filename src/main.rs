use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Json, Executor};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    direction: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password123@localhost/test")
        .await?;

    pool.execute("CREATE TABLE IF NOT EXISTS people (name text, age int, direction text)")
        .await?;

    let person = Person {
        name: "John".to_string(),
        age: 30,
        direction: "USA".to_string(),
    };

    sqlx::query("INSERT INTO people SELECT * FROM jsonb_populate_record(NULL::people, $1)")
        .bind(Json(&person))
        .execute(&pool)
        .await?;

    Ok(())
}
