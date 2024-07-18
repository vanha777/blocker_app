use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::{error::Error, fmt::format};

use crate::Endpoint;

pub async fn send(endpoint: &Endpoint) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut url = endpoint.endpoint.clone().unwrap();
    // build header
    let mut headers = HeaderMap::new();
    match endpoint.header.clone() {
        Some(x) => {
            for (key, value) in x {
                headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                );
            }
        }
        None => (),
    }
    // build body if exsist
    let body = endpoint.body.clone();
    // build query if exsist
    let query = match endpoint.query.clone() {
        Some(x) => {
            let mut query = String::new();
            for (k, v) in endpoint.query.clone().unwrap() {
                let new_query = format!("{k}={v}");
                query = format!("{query}&{new_query}");
            }
            Some(query)
        }
        None => None,
    };
    // add query to url if exsist
    match query {
        Some(x) => url = format!("{url}?{x}"),
        None => (),
    };

    match endpoint.method.clone().unwrap().as_str() {
        "GET" => {
            let response = client
                .get(url)
                .headers(headers)
                .send()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "POST" => {
            let response = client
                .post(url)
                .headers(headers)
                .json(&body)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "PUT" => {
            let response = client
                .put(url)
                .headers(headers)
                .json(&body)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "DELETE" => {
            let response = client
                .delete(url)
                .headers(headers)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        _ => Err("Api Method are not yet supported".to_string()),
    }
}

fn construct_headers(subscription_key: &str, fred_api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert(
        "Ocp-Apim-Subscription-Key",
        HeaderValue::from_str(subscription_key).unwrap(),
    );
    headers.insert("FredApiKey", HeaderValue::from_str(fred_api_key).unwrap());

    headers
}

pub async fn send_dummy_data(
    url: &str,
    subscription_key: &str,
    fred_api_key: &str,
    payload: &str,
) -> Result<(), Box<dyn Error>> {
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
