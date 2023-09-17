use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Whether to fetch all activities or only those without defaults
    #[serde(rename = "nodefaults")]
    pub r#nodefaults: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItem {
    /// The plugin name of the activity
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The display name of the activity
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
}

pub type r#ReturnsOptions = Vec<ReturnsOptionsItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "options")]
    pub r#options: ReturnsOptions,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_get_activity_options", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
