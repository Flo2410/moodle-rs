use serde::{self, Deserialize, Serialize};

/// List of course id. If empty return all courses

/// except front page course.
pub type r#ParamsOptionsIds = Vec<Option<i64>>;

/// options - operator OR is used
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptions {
    /// List of course id. If empty return all courses except front page course.
    #[serde(rename = "ids")]
    pub r#ids: ParamsOptionsIds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// options - operator OR is used
    #[serde(rename = "options")]
    pub r#options: ParamsOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCourseformatoptionsItem {
    /// course format option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// course format option value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// additional options for particular course format
pub type r#ReturnsItemCourseformatoptions = Vec<ReturnsItemCourseformatoptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCustomfieldsItem {
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// The type of the custom field - text, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The raw value of the custom field
    #[serde(rename = "valueraw")]
    pub r#valueraw: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Custom fields and associated values
pub type r#ReturnsItemCustomfields = Vec<ReturnsItemCustomfieldsItem>;

/// course
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course short name
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// sort order into the category
    #[serde(rename = "categorysortorder")]
    pub r#categorysortorder: Option<i64>,
    /// full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// course display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// summary
    #[serde(rename = "summary")]
    pub r#summary: Option<String>,
    /// summary format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "summaryformat")]
    pub r#summaryformat: Option<i64>,
    /// course format: weeks, topics, social, site,..
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// 1 if grades are shown, otherwise 0
    #[serde(rename = "showgrades")]
    pub r#showgrades: Option<i64>,
    /// number of recent items appearing on the course page
    #[serde(rename = "newsitems")]
    pub r#newsitems: Option<i64>,
    /// timestamp when the course start
    #[serde(rename = "startdate")]
    pub r#startdate: Option<i64>,
    /// timestamp when the course end
    #[serde(rename = "enddate")]
    pub r#enddate: Option<i64>,
    /// (deprecated, use courseformatoptions) number of weeks/topics
    #[serde(rename = "numsections")]
    pub r#numsections: Option<i64>,
    /// largest size of file that can be uploaded into the course
    #[serde(rename = "maxbytes")]
    pub r#maxbytes: Option<i64>,
    /// are activity report shown (yes = 1, no =0)
    #[serde(rename = "showreports")]
    pub r#showreports: Option<i64>,
    /// 1: available to student, 0:not available
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// (deprecated, use courseformatoptions) How the hidden sections in the course are displayed to students
    #[serde(rename = "hiddensections")]
    pub r#hiddensections: Option<i64>,
    /// no group, separate, visible
    #[serde(rename = "groupmode")]
    pub r#groupmode: Option<i64>,
    /// 1: yes, 0: no
    #[serde(rename = "groupmodeforce")]
    pub r#groupmodeforce: Option<i64>,
    /// default grouping id
    #[serde(rename = "defaultgroupingid")]
    pub r#defaultgroupingid: Option<i64>,
    /// timestamp when the course have been created
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timestamp when the course have been modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Enabled, control via completion and activity settings. Disbaled, not shown in activity settings.
    #[serde(rename = "enablecompletion")]
    pub r#enablecompletion: Option<i64>,
    /// 1: yes 0: no
    #[serde(rename = "completionnotify")]
    pub r#completionnotify: Option<i64>,
    /// forced course language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// name of the force theme
    #[serde(rename = "forcetheme")]
    pub r#forcetheme: Option<String>,
    /// additional options for particular course format
    #[serde(rename = "courseformatoptions")]
    pub r#courseformatoptions: ReturnsItemCourseformatoptions,
    /// Whether the activity dates are shown or not
    #[serde(rename = "showactivitydates")]
    pub r#showactivitydates: Option<bool>,
    /// Whether the activity completion conditions are shown or not
    #[serde(rename = "showcompletionconditions")]
    pub r#showcompletionconditions: Option<bool>,
    /// Custom fields and associated values
    #[serde(rename = "customfields")]
    pub r#customfields: ReturnsItemCustomfields,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_get_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
