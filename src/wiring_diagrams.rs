use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SystemTypesText {
    system_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SystemType {
    system_id: String, // This one is necessary for /diagram
    text: Option<SystemTypesText>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemGroupText {
    pub title: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemGroup {
    system_types: Vec<SystemType>,
    pub text: Option<SystemGroupText>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemGroupTree {
    pub system_groups: Vec<SystemGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiringDiagramsResponse {
    pub system_group_tree: SystemGroupTree,
}

#[derive(Serialize)]
pub struct WiringDiagramsStructBody {
    #[serde(rename = "kType")]
    ktype: String,
    language: String,
}

pub fn get_wiring_diagram_systems_response(
    token: &str,
    ktype: &str,
    language: &str,
) -> Result<Vec<WiringDiagramsResponse>, reqwest::Error> {
    // Define the body to send
    let body = WiringDiagramsStructBody {
        ktype: ktype.to_string(),
        language: language.to_string(),
    };

    // Create the request
    let client = Client::new();
    let response = client
        .post("https://rmi.hgs.cloud/api/v1/wiring-diagrams/systems")
        .json(&body)
        .bearer_auth(token)
        .send()?
        .error_for_status()?
        .json::<Vec<WiringDiagramsResponse>>()?;

    Ok(response)
}

pub fn select_wiring_diagram_system(gwdsr: Vec<WiringDiagramsResponse>) {
    println!("\nSelecciona un esquema eléctrico:");
    for item in &gwdsr {
        for (i, group) in item.system_group_tree.system_groups.iter().enumerate() {
            println!("{0} - {1}", i + 1, group.text.as_ref().unwrap().title);
        }
    }
}