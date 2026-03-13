use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HellaToken {
    id_token: String,
    // expires_in: u32,
}

pub fn get_client_token(
    auth_url: &str,
    username: String,
    password: String,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(auth_url)
        .basic_auth(username, Some(password))
        .send()?
        .error_for_status()?
        .json::<HellaToken>()?;

    Ok(response.id_token)
}
