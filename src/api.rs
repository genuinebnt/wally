use crate::{cli::Params, model::{WallhavenResponse, Wallpaper}};

pub async fn fetch_wallpapers(params: Params, count: usize) -> Result<Vec<Wallpaper>, reqwest::Error> {
    let mut response: Vec<Wallpaper> = Vec::new();    
    let mut handles = vec![];
    for i in 0..count {
        let params = params.clone();
        let handle = tokio::spawn(async move {
            fetch_wallpaper_per_page(i, params).await.unwrap()
        });
        handles.push(handle);
    }
    for handle in handles {
        response.extend(handle.await.unwrap().data);
    }

    Ok(response)
}

pub async fn fetch_wallpaper_per_page(
    page: usize,
    params: Params,
) -> Result<WallhavenResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://wallhaven.cc/api/v1/search?apiKey={}&q={}&page={}&categories={}&sorting={}&order={}&atleast={}&ratios={}&purity={}",
            params.api_key.unwrap(),
            params.query.unwrap(),
            page,
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
