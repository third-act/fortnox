use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub customer_number: Option<String>,

    pub name: String,

    pub address1: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub address2: Option<String>,

    pub zip_code: String,

    pub city: String,

    pub country_code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comments: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub phone1: Option<String>,
}
