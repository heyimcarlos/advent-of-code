async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::COOKIE, "session=53616c7465645f5fb7ed99a37c6d30273f56ef9dc63fc6089c05d0fe59c2eaa39164e5b93634fd5e381afc6970e822a2ebd062eeb861e2da2abcd9548afc140b;")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    let res = runtime.block_on(get(url))?;
    Ok(res)
}
