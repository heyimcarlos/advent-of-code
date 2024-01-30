pub async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    // let runtime = tokio::runtime::

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_static("session="),
    );

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

// pub fn
