pub async fn hex(url: String) -> Result<String, String> {
    let html = get_html_from_url(url).await?;
    let result = scrapy::extract_ingredients(&html);

    let joined = result.join(",");

    Ok(joined)
}

async fn get_html_from_url(url: String) -> Result<String, String> {
    let x = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let text = x.text().await.map_err(|e| e.to_string())?;

    Ok(text)
}
