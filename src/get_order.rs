use crate::{Error, Gateway, Order, BASE_URL};
use serde::Deserialize;

impl Gateway {
    pub async fn get_order<'a>(&self, document_number: &'a str) -> Result<Order, Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            order: Order,
        }

        let url = format!("{}/orders/{}", BASE_URL, &document_number);
        let res: Response = self.get(&url).await?;
        Ok(res.order)
    }
}
