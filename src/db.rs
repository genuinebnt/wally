use sqlx::{PgPool, Pool, Postgres};

use crate::model::Wallpaper;

pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn bulk_insert_wallpapers(
        &self,
        wallpapers: Vec<Wallpaper>,
    ) -> Result<(), sqlx::Error> {
        let mut insert_query: String = "
        INSERT INTO WALLPAPERS (
            ID, URL, SHORT_URL, VIEWS, FAVORITES, SOURCE, PURITY, CATEGORY,
            DIMENSION_X, DIMENSION_Y, RESOLUTION, RATIO, FILE_SIZE, CREATED_AT,
            COLORS, PATH, THUMB_LARGE, THUMB_ORIGINAL, THUMB_SMALL
        )
        VALUES "
            .to_string();

        let values: String = wallpapers.iter().map(|wallpaper| format!(
            "('{}', '{}', '{}', {}, {}, '{}', '{}', '{}', {}, {}, '{}', '{}', {}, '{}', array[{}], '{}', '{}', '{}', '{}')",
            wallpaper.id,
            wallpaper.url,
            wallpaper.short_url,
            wallpaper.views,
            wallpaper.favorites,
            wallpaper.source,
            wallpaper.purity,
            wallpaper.category,
            wallpaper.dimension_x,
            wallpaper.dimension_y,
            wallpaper.resolution,
            wallpaper.ratio,
            wallpaper.file_size,
            wallpaper.created_at,
            wallpaper.colors.iter().map(|color| format!("'{}'", color)).collect::<Vec<_>>().join(", "),
            wallpaper.path,
            wallpaper.thumbs.large,
            wallpaper.thumbs.original,
            wallpaper.thumbs.small
        )).collect::<Vec<_>>().join(", ");

        insert_query.push_str(&values);
        insert_query.push_str("ON CONFLICT(id) DO NOTHING;");
        sqlx::query(&insert_query).execute(&self.pool).await?;
        Ok(())
    }
}
