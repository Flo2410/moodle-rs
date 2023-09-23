use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPreferencesItem {
    /// The name of the preference
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the preference, do not set this field if you want to remove (unset) the current value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User preferences
pub type r#ParamsPreferences = Vec<ParamsPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, default to current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Enable or disable notifications for this user
    #[serde(rename = "emailstop")]
    pub r#emailstop: Option<i64>,
    /// User preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ParamsPreferences>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// timestamp when the course start
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// timestamp when the course end
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// 1: available to student, 0:not available
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// Whether the activity dates are shown or not
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// Whether the activity completion conditions are shown or not
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// course display name
    #[serde(rename = "fullnamedisplay")]
    pub r#fullnamedisplay: Option<String>,
    /// course url
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// course image
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
    /// course has progress
    #[serde(rename = "hasprogress")]
    pub r#hasprogress: Option<bool>,
    /// course is favourite
    #[serde(rename = "isfavourite")]
    pub r#isfavourite: Option<bool>,
    /// course is hidden
    #[serde(rename = "hidden")]
    pub r#hidden: Option<bool>,
    /// course show shortname
    #[serde(rename = "showshortname")]
    pub r#showshortname: Option<bool>,
    /// course category
    #[serde(rename = "coursecategory")]
    pub r#coursecategory: Option<String>,
}

pub type r#ReturnsCourses = Vec<ReturnsCoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDataItem {
    /// courses
    #[serde(rename = "courses")]
    pub r#courses: Option<r#ReturnsCourses>,
    /// next offset
    #[serde(rename = "nextoffset")]
    pub r#nextoffset: Option<i64>,
}

pub type r#ReturnsData = Vec<ReturnsDataItem>;

pub type r#ReturnsError = bool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Preferences saved
    #[serde(rename = "error")]
    pub r#error: Option<r#ReturnsError>,
    /// list of warnings
    #[serde(rename = "data")]
    pub r#data: Option<r#ReturnsData>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_user_update_user_preferences", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_user_update_user_preferences", params)
        .await
}
