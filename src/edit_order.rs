use crate::{Error, Gateway, Order, BASE_URL};
use serde::{Deserialize, Serialize};

impl Gateway {
    pub async fn edit_order<'a>(&self, order: &'a Order) -> Result<Order, Error> {
        let document_number = match &order.document_number {
            Some(document_number) => document_number,
            None => {
                return Err(Error::SerializationError(format!(
                    "Cannot edit an order with no document number."
                )))
            }
        };

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Request<'a> {
            order: &'a Order,
        }

        let body = Request { order: &order };

        let url = format!("{}/orders/{}", BASE_URL, document_number);

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            order: Order,
        }

        let res: Response = self.put(&url, &body).await?;
        Ok(res.order)
    }
}
