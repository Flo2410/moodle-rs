use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Assessment id.
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// The feedback for the reviewer.
    #[serde(rename = "feedbacktext")]
    pub r#feedbacktext: Option<String>,
    /// The feedback format for text.
    #[serde(rename = "feedbackformat")]
    pub r#feedbackformat: Option<i64>,
    /// The new weight for the assessment.
    #[serde(rename = "weight")]
    pub r#weight: Option<i64>,
    /// The new grading grade.
    #[serde(rename = "gradinggradeover")]
    pub r#gradinggradeover: Option<String>,
}

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
    /// status: true if the assessment was evaluated, false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_evaluate_assessment", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
