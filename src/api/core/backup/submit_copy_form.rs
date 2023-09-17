use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The data from the create copy form, encoded as a json array
    #[serde(rename = "jsonformdata")]
    pub r#jsonformdata: Option<String>,
}
