use crate::{Currency, Error, Gateway, Order, OrderRow, BASE_URL};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

impl Gateway {
    pub async fn cancel_order<'a>(&self, document_number: &'a str) -> Result<Order, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Request {}

        let body = Request {};

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        struct Order2 {
            pub document_number: String,

            pub customer_number: String,

            pub order_rows: Vec<Row2>,

            pub currency: Currency,

            #[serde(rename = "VATIncluded")]
            pub vat_included: bool,

            pub comments: Option<String>,

            pub delivery_date: NaiveDate,

            pub order_date: NaiveDate,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(rename_all = "PascalCase")]
        struct Row2 {
            pub article_number: String,
            pub ordered_quantity: String,
            pub delivered_quantity: f64,
            pub description: String,
            pub price: f64,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            order: Order2,
        }

        let url = format!("{}/orders/{}/cancel", BASE_URL, &document_number);
        let res: Response = self.put(&url, &body).await?;

        // Make order from order with float deliveredquantity.
        let mut rows = vec![];
        for row in res.order.order_rows {
            rows.push(OrderRow {
                article_number: Some(row.article_number),
                ordered_quantity: row.ordered_quantity,
                delivered_quantity: row.delivered_quantity.to_string(),
                description: row.description,
                price: row.price,
            });
        }
        let order = Order {
            document_number: Some(res.order.document_number),
            customer_number: res.order.customer_number,
            order_rows: rows,
            currency: res.order.currency,
            vat_included: res.order.vat_included,
            comments: res.order.comments,
            delivery_date: res.order.delivery_date,
            order_date: res.order.order_date,
        };

        Ok(order)
    }
}
