use reqwest::header::CONTENT_TYPE;
use serde_json::json;

pub static AUTH_URL: &str =
    "https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token";
pub static URL_HEAD: &str = "https://tdx.transportdata.tw/api/basic";

pub async fn get_token_header(client_id: &str, client_secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let auth_header = json!({
        "grant_type": "client_credentials",
        "client_id": client_id,
        "client_secret": client_secret
    });

    let auth_response = client
        .post(AUTH_URL)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&auth_header)
        .send()
        .await?
        .text()
        .await?;

        let data_header = auth_response.split_once("\":\"").unwrap().1;

        match data_header.split_once("\",") {
            Some((token, _)) => Ok(format!("Bearer {}", token)),
            None => Err("Token not found".into()),
        }
}