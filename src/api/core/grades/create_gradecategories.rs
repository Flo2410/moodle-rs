use serde::{self, Deserialize, Serialize};

/// optional category data
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCategoriesItemOptions {
    /// aggregation method
    #[serde(rename = "aggregation")]
    pub r#aggregation: Option<i64>,
    /// exclude empty grades
    #[serde(rename = "aggregateonlygraded")]
    pub r#aggregateonlygraded: Option<bool>,
    /// aggregate outcomes
    #[serde(rename = "aggregateoutcomes")]
    pub r#aggregateoutcomes: Option<bool>,
    /// drop low grades
    #[serde(rename = "droplow")]
    pub r#droplow: Option<i64>,
    /// the category total name
    #[serde(rename = "itemname")]
    pub r#itemname: Option<String>,
    /// the category iteminfo
    #[serde(rename = "iteminfo")]
    pub r#iteminfo: Option<String>,
    /// the category idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// the grade type
    #[serde(rename = "gradetype")]
    pub r#gradetype: Option<i64>,
    /// the grade max
    #[serde(rename = "grademax")]
    pub r#grademax: Option<i64>,
    /// the grade min
    #[serde(rename = "grademin")]
    pub r#grademin: Option<i64>,
    /// the grade to pass
    #[serde(rename = "gradepass")]
    pub r#gradepass: Option<i64>,
    /// the display type
    #[serde(rename = "display")]
    pub r#display: Option<i64>,
    /// the decimal count
    #[serde(rename = "decimals")]
    pub r#decimals: Option<i64>,
    /// grades hidden until
    #[serde(rename = "hiddenuntil")]
    pub r#hiddenuntil: Option<i64>,
    /// lock grades after
    #[serde(rename = "locktime")]
    pub r#locktime: Option<i64>,
    /// weight adjusted
    #[serde(rename = "weightoverride")]
    pub r#weightoverride: Option<bool>,
    /// weight coefficient
    #[serde(rename = "aggregationcoef2")]
    pub r#aggregationcoef2: Option<String>,
    /// The parent category id
    #[serde(rename = "parentcategoryid")]
    pub r#parentcategoryid: Option<i64>,
    /// the parent category idnumber
    #[serde(rename = "parentcategoryidnumber")]
    pub r#parentcategoryidnumber: Option<String>,
}

/// Category to create
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCategoriesItem {
    /// fullname of category
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// optional category data
    #[serde(rename = "options")]
    pub r#options: ParamsCategoriesItemOptions,
}

/// Categories to create
pub type r#ParamsCategories = Vec<ParamsCategoriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Categories to create
    #[serde(rename = "categories")]
    pub r#categories: ParamsCategories,
}

pub type r#ReturnsCategoryids = Vec<Option<i64>>;

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
    #[serde(rename = "categoryids")]
    pub r#categoryids: ReturnsCategoryids,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_grades_create_gradecategories", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
