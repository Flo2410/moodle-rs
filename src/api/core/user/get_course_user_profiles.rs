use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUserlistItem {
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

pub type r#ParamsUserlist = Vec<ParamsUserlistItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "userlist")]
    pub r#userlist: ParamsUserlist,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCustomfieldsItem {
    /// The type of the custom field - text field, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the custom field (as stored in the database)
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// The value of the custom field for display
    #[serde(rename = "displayvalue")]
    pub r#displayvalue: Option<String>,
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field - to be able to build the field class in the code
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// User custom fields (also known as user profile fields)
pub type r#ReturnsItemCustomfields = Vec<ReturnsItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemPreferencesItem {
    /// The name of the preferences
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the preference
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Users preferences
pub type r#ReturnsItemPreferences = Vec<ReturnsItemPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemGroupsItem {
    /// group id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// group name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// group description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
}

/// user groups
pub type r#ReturnsItemGroups = Vec<ReturnsItemGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemRolesItem {
    /// role id
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
    /// role name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// role shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// role sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
}

/// user roles
pub type r#ReturnsItemRoles = Vec<ReturnsItemRolesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemEnrolledcoursesItem {
    /// Id of the course
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Fullname of the course
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Shortname of the course
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
}

/// Courses where the user is enrolled - limited by which courses the user is able to see
pub type r#ReturnsItemEnrolledcourses = Vec<ReturnsItemEnrolledcoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the user
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The username
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// The first name(s) of the user
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// The family name of the user
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
    /// The fullname of the user
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// An email address - allow email as root@localhost
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Postal address
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Phone 1
    #[serde(rename = "phone1")]
    pub r#phone1: Option<String>,
    /// Phone 2
    #[serde(rename = "phone2")]
    pub r#phone2: Option<String>,
    /// department
    #[serde(rename = "department")]
    pub r#department: Option<String>,
    /// institution
    #[serde(rename = "institution")]
    pub r#institution: Option<String>,
    /// An arbitrary ID code number perhaps from the institution
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// user interests (separated by commas)
    #[serde(rename = "interests")]
    pub r#interests: Option<String>,
    /// first access to the site (0 if never)
    #[serde(rename = "firstaccess")]
    pub r#firstaccess: Option<i64>,
    /// last access to the site (0 if never)
    #[serde(rename = "lastaccess")]
    pub r#lastaccess: Option<i64>,
    /// Auth plugins include manual, ldap, etc
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// Suspend user account, either false to enable user login or true to disable it
    #[serde(rename = "suspended")]
    pub r#suspended: Option<bool>,
    /// Active user: 1 if confirmed, 0 otherwise
    #[serde(rename = "confirmed")]
    pub r#confirmed: Option<bool>,
    /// Language code such as "en", must exist on server
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// Calendar type such as "gregorian", must exist on server
    #[serde(rename = "calendartype")]
    pub r#calendartype: Option<String>,
    /// Theme name such as "standard", must exist on server
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Timezone code such as Australia/Perth, or 99 for default
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
    /// Mail format code is 0 for plain text, 1 for HTML etc
    #[serde(rename = "mailformat")]
    pub r#mailformat: Option<i64>,
    /// User profile description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// int format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// Home city of the user
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Home country code of the user, such as AU or CZ
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// User image profile URL - small version
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
    /// User image profile URL - big version
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// User custom fields (also known as user profile fields)
    #[serde(rename = "customfields")]
    pub r#customfields: ReturnsItemCustomfields,
    /// Users preferences
    #[serde(rename = "preferences")]
    pub r#preferences: ReturnsItemPreferences,
    /// user groups
    #[serde(rename = "groups")]
    pub r#groups: ReturnsItemGroups,
    /// user roles
    #[serde(rename = "roles")]
    pub r#roles: ReturnsItemRoles,
    /// Courses where the user is enrolled - limited by which courses the user is able to see
    #[serde(rename = "enrolledcourses")]
    pub r#enrolledcourses: ReturnsItemEnrolledcourses,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_user_get_course_user_profiles", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
