use crate::{Error, Gateway, Order, BASE_URL};
use serde::{Deserialize, Serialize};

impl Gateway {
    pub async fn add_order<'a>(&self, order: &'a Order) -> Result<Order, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Request<'a> {
            order: &'a Order,
        }

        let body = Request { order: &order };

        let url = format!("{}/orders", BASE_URL);

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            order: Order,
        }

        let res: Response = self.post(&url, &body).await?;
        Ok(res.order)
    }
}
