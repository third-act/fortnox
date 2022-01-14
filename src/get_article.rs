use crate::{Article, Error, Gateway, BASE_URL};
use serde::Deserialize;

impl Gateway {
    pub async fn get_article<'a>(&self, article_number: &'a str) -> Result<Article, Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            article: Article,
        }

        let url = format!("{}/articles/{}", BASE_URL, &article_number);
        let res: Response = self.get(&url).await?;
        Ok(res.article)
    }
}
