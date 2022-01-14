use crate::{Customer, Error, Gateway, BASE_URL};
use serde::{Deserialize, Serialize};

impl Gateway {
    pub async fn add_customer<'a>(&self, customer: &'a Customer) -> Result<Customer, Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Request<'a> {
            customer: &'a Customer,
        }

        let body = Request {
            customer: &customer,
        };

        let url = format!("{}/customers", BASE_URL);

        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            customer: Customer,
        }

        let res: Response = self.post(&url, &body).await?;
        Ok(res.customer)
    }
}
