use crate::{Customer, Error, Gateway, BASE_URL};
use serde::Deserialize;

impl Gateway {
    pub async fn get_customer<'a>(&self, customer_number: &'a str) -> Result<Customer, Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            customer: Customer,
        }

        let url = format!("{}/customers/{}", BASE_URL, &customer_number);
        let res: Response = self.get(&url).await?;
        Ok(res.customer)
    }
}
