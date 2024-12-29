use sqlx::{PgPool, Pool, Postgres};

pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn insert_index(&self, index: i32) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO indexes (index) VALUES ($1)")
            .bind(index)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}