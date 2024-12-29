use serde::Deserialize;

use crate::cli::Params;

#[derive(Debug, Deserialize)]
pub struct WallhavenResponse {
    pub data: Vec<Wallpaper>,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub views: u32,
    pub favorites: u32,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: u32,
    pub dimension_y: u32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u32,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub current_page: u32,
    pub last_page: u32,
    pub per_page: u32,
    pub total: u32,
    pub query: Option<String>,
    pub seed: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

pub async fn fetch_wallpapers(params: Params) -> Result<WallhavenResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://wallhaven.cc/api/v1/search?apiKey={}&q={}&page={}&categories={}&sorting={}&order={}&atleast={}&ratios={}&purity={}",
            params.api_key.unwrap(),
            params.query.unwrap(),
            params.page.unwrap(),
            params.category.unwrap(),
            params.sort.unwrap(),
            params.order.unwrap(),
            params.atleast.unwrap(),
            params.ratio.unwrap(),
            params.purity.unwrap()
        ))
        .send()
        .await?
        .json::<WallhavenResponse>()
        .await?;
    Ok(response)
}