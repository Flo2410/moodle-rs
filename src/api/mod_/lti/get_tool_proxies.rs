use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Orphaned tool types only
    #[serde(rename = "orphanedonly")]
    pub r#orphanedonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Tool proxy id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Tool proxy name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Tool proxy registration URL
    #[serde(rename = "regurl")]
    pub r#regurl: Option<String>,
    /// Tool proxy state
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
    /// Tool proxy globally unique identifier
    #[serde(rename = "guid")]
    pub r#guid: Option<String>,
    /// Tool proxy shared secret
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
    /// Tool proxy consumer code
    #[serde(rename = "vendorcode")]
    pub r#vendorcode: Option<String>,
    /// Tool proxy capabilities offered
    #[serde(rename = "capabilityoffered")]
    pub r#capabilityoffered: Option<String>,
    /// Tool proxy services offered
    #[serde(rename = "serviceoffered")]
    pub r#serviceoffered: Option<String>,
    /// Tool proxy
    #[serde(rename = "toolproxy")]
    pub r#toolproxy: Option<String>,
    /// Tool proxy time created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Tool proxy modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lti_get_tool_proxies", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
