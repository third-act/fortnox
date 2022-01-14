use crate::Currency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub document_number: Option<String>,

    pub customer_number: String,

    pub order_rows: Vec<Row>,

    pub currency: Currency,

    #[serde(rename = "VATIncluded")]
    pub vat_included: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comments: Option<String>,

    pub delivery_date: NaiveDate, // "2006-01-02"

    pub order_date: NaiveDate, // "2006-01-02"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Row {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub article_number: Option<String>,

    pub ordered_quantity: String,

    pub delivered_quantity: String,

    pub description: String,

    pub price: f64,
}
