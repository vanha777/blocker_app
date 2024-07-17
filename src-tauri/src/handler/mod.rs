use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::error::Error;

pub async fn send_query(url: &str, query: &[(&str, &str)], subscription_key: &str, fred_api_key: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut full_url = reqwest::Url::parse(url)?;
    full_url.query_pairs_mut().extend_pairs(query);

    let headers = construct_headers(subscription_key, fred_api_key);

    let response = client
        .get(full_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    println!("Response data: {}", response);

    Ok(())
}

fn construct_headers(subscription_key: &str, fred_api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("Ocp-Apim-Subscription-Key", HeaderValue::from_str(subscription_key).unwrap());
    headers.insert("FredApiKey", HeaderValue::from_str(fred_api_key).unwrap());

    headers
}

pub async fn send_dummy_data(url: &str, subscription_key: &str, fred_api_key: &str, payload: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let headers = construct_headers(subscription_key, fred_api_key);

    let response = client
        .post(url)
        .headers(headers)
        .body(payload.to_string())
        .send()
        .await?
        .text()
        .await?;

    println!("Response data: {}", response);

    Ok(())
}