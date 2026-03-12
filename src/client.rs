use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HellaToken {
    id_token: String,
    token_type: String,
    expires_in: u32,
}

pub fn get_client_token(
    auth_url: &str,
    username: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post(auth_url)
        .basic_auth(username, Some(password))
        .send()?
        .json::<HellaToken>()?;

    return Ok(response.id_token);
}
