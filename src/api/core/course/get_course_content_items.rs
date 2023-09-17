use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// ID of the course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsContentItemsItem {
    /// The id of the content item
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Name of the content item
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The string title of the content item, human readable
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// The link to the content item creation page
    #[serde(rename = "link")]
    pub r#link: Option<String>,
    /// Html containing the icon for the content item
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// Html description / help for the content item
    #[serde(rename = "help")]
    pub r#help: Option<String>,
    /// The archetype of the module exposing the content item
    #[serde(rename = "archetype")]
    pub r#archetype: Option<String>,
    /// The name of the component exposing the content item
    #[serde(rename = "componentname")]
    pub r#componentname: Option<String>,
    /// The purpose of the component exposing the content item
    #[serde(rename = "purpose")]
    pub r#purpose: Option<String>,
    /// Has the user favourited the content item
    #[serde(rename = "favourite")]
    pub r#favourite: Option<bool>,
    /// If this item was pulled from the old callback and has no item id.
    #[serde(rename = "legacyitem")]
    pub r#legacyitem: Option<bool>,
    /// Has this item been recommended
    #[serde(rename = "recommended")]
    pub r#recommended: Option<bool>,
}

pub type r#ReturnsContentItems = Vec<ReturnsContentItemsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "content_items")]
    pub r#content_items: ReturnsContentItems,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_course_get_course_content_items", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
