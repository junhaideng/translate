use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Response {
    #[serde(rename = "translation", default)]
    pub translation: Vec<String>,

    #[serde(rename = "basic", default)]
    pub basic: Basic,

    #[serde(rename = "query", default)]
    pub query: String,

    #[serde(rename = "errorCode", default)]
    pub error_code: i64,

    #[serde(rename = "web", default)]
    pub web: Vec<Web>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Basic {
    #[serde(rename = "us-phonetic", default)]
    pub us_phonetic: String,

    #[serde(rename = "phonetic", default)]
    pub phonetic: String,

    #[serde(rename = "uk-phonetic", default)]
    pub uk_phonetic: String,

    #[serde(rename = "explains", default)]
    pub explains: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Web {
    #[serde(rename = "value", default)]
    pub value: Vec<String>,

    #[serde(rename = "key", default)]
    pub key: String,
}
