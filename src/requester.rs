use dotenv::dotenv;
use std::env;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

const API_BASE_URL: &str = "https://api.openai.com";
const API_VERSION: &str = "v1";

async fn request<T>(method: String, url: String, body: Option<HashMap<&str, Value>>) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("Please define openai api key");

    let response;
    if method == "POST" {
        let client = reqwest::Client::new();
        response = client.post(url)
            .header("Content-type", "application/json")
            .header("Authorization", "Bearer ".to_owned() + &api_key)
            .json(&body)
            .send()
            .await;
    } else {
        // Assume GET
        let client = reqwest::Client::new();
        response = client.get(url)
            .header("Content-type", "application/json")
            .header("Authorization", "Bearer ".to_owned() + &api_key)
            .send()
            .await;
    }

    match &response {
        Ok(r) => {
            println!("{:?}", r.status());
            if r.status() != StatusCode::OK {
                return Err(r.status());
            } else {
                let content = response.unwrap().json::<T>().await;
                match content {
                    Ok(s) => Ok(s),
                    Err(e) => {
                        println!("{:?}", e);
                        Err(StatusCode::BAD_REQUEST)
                    }
                }
            }
        }
        Err(e) => {
            println!("{} - {:?}", e.is_status(), e.status());
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }
}

/// Handles requests for the `/completions` endpoint
pub async fn completions<T>(arguments: HashMap<&str, Value>) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    let url = format!("{}/{}/completions", API_BASE_URL, API_VERSION);
    request(String::from("POST"), url, Some(arguments)).await
}

/// Handles requests for the `/edits` endpoint
pub async fn edits<T>(arguments: HashMap<&str, Value>) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    let url = format!("{}/{}/edits", API_BASE_URL, API_VERSION);
    request(String::from("POST"), url, Some(arguments)).await
}

#[derive(strum_macros::Display)]
pub enum ImageRequestType {
    #[strum(serialize = "generations")]
    Generations,
    #[strum(serialize = "edits")]
    Edits,
    #[strum(serialize = "variations")]
    Variations
}

/// Handles requests for the `/images` endpoint
pub async fn images<T>(request_type: ImageRequestType, arguments: HashMap<&str, Value>) -> Result<T, StatusCode>
where
T: DeserializeOwned
{
    let url = format!("{}/{}/images/{}", API_BASE_URL, API_VERSION, request_type.to_string());
    request(String::from("POST"), url, Some(arguments)).await
}

/// Handles requests for the `/models` endpoint
pub async fn models<T>(model_name: Option<String>) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    let mut url = format!("{}/{}/models", API_BASE_URL, API_VERSION);
    if model_name != None {
        url = format!("{}/{}/models/{}", API_BASE_URL, API_VERSION, model_name.unwrap());
    }
    request(String::from("GET"), url, None).await
}