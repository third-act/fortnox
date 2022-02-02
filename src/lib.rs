mod article;
pub use article::Article;
mod customer;
pub use customer::Customer;
mod currency;
pub use currency::Currency;
mod order;
pub use order::{Order, Row as OrderRow};
mod add_customer;
mod add_order;
mod api_error_code;
mod cancel_order;
mod get_article;
mod get_customer;
mod get_order;
pub use api_error_code::ApiErrorCode;
mod error;
pub use error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

const BASE_URL: &str = "https://api.fortnox.se/3";
const INITIAL_DELAY_MS: f64 = 100.0;
const RETRIES: u8 = 5;
const BACKOFF: f64 = 2.0;

pub struct Gateway {
    client: reqwest::Client,
}

impl Gateway {
    pub async fn new(
        _client_id: String,
        token: String,
        secret: String,
        timeout: Option<Duration>,
    ) -> Result<Gateway, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let access_token_header = match reqwest::header::HeaderValue::from_str(&token) {
            Ok(header) => header,
            Err(err) => {
                return Err(Error::Unspecified(format!(
                    "Could not create auth header ({}).",
                    err.to_string()
                )))
            }
        };
        headers.insert("Access-Token", access_token_header);

        let client_secret_header = match reqwest::header::HeaderValue::from_str(&secret) {
            Ok(header) => header,
            Err(err) => {
                return Err(Error::Unspecified(format!(
                    "Could not create auth header ({}).",
                    err.to_string()
                )))
            }
        };
        headers.insert("Client-Secret", client_secret_header);

        let timeout = match timeout {
            Some(t) => t,
            None => Duration::new(60, 0),
        };

        let client = match reqwest::ClientBuilder::new()
            .default_headers(headers)
            .https_only(true)
            .timeout(timeout)
            .build()
        {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::Unspecified(format!(
                    "Could not create reqwest client ({}).",
                    err.to_string()
                )))
            }
        };

        let c = Gateway { client };
        Ok(c)
    }

    async fn post<'a, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let mut delay = INITIAL_DELAY_MS;
        for _ in 0..RETRIES {
            let res = match self.post_without_retry(url, body).await {
                Ok(res) => res,
                Err(err) => {
                    delay = self.randomized_exponential_backoff(delay).await;

                    match err {
                        Error::Throttling => {
                            continue;
                        }
                        _ => return Err(err),
                    };
                }
            };

            return Ok(res);
        }

        Err(Error::Throttling)
    }

    async fn post_without_retry<'a, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let res = match self.client.post(url).json(body).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send request ({}).",
                    err.to_string()
                )))
            }
        };

        let status = res.status().as_u16();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| String::from("Could not retrieve body text."));

        if status < 200 || status > 299 {
            if status == 429 {
                return Err(Error::Throttling);
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ErrorInformation {
                pub error_information: ApiError,
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ApiError {
                pub error: u32,
                pub message: String,
                pub code: ApiErrorCode,
            }

            let api_error: ErrorInformation =
                serde_json::from_str(&text).unwrap_or_else(|_| ErrorInformation {
                    error_information: ApiError {
                        error: 0,
                        message: format!("Unknown error ({}: {})", status, text),
                        code: ApiErrorCode::Unknown,
                    },
                });
            return Err(Error::ApiError(
                api_error.error_information.code,
                api_error.error_information.message,
            ));
        }

        let body: T = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::SerializationError(format!(
                    "Could not deserialize response from \"{}\" ({}).",
                    text,
                    err.to_string()
                )))
            }
        };
        Ok(body)
    }

    async fn get<'a, T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let mut delay = INITIAL_DELAY_MS;
        for _ in 0..RETRIES {
            let res = match self.get_without_retry(url).await {
                Ok(res) => res,
                Err(err) => {
                    delay = self.randomized_exponential_backoff(delay).await;

                    match err {
                        Error::Throttling => {
                            continue;
                        }
                        _ => return Err(err),
                    };
                }
            };

            return Ok(res);
        }

        Err(Error::Throttling)
    }

    async fn get_without_retry<'a, T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let res = match self.client.get(url).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send request ({}).",
                    err.to_string()
                )))
            }
        };

        let status = res.status().as_u16();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| String::from("Could not retrieve body text."));

        if status < 200 || status > 299 {
            if status == 429 {
                return Err(Error::Throttling);
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ErrorInformation {
                pub error_information: ApiError,
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ApiError {
                pub error: u32,
                pub message: String,
                pub code: ApiErrorCode,
            }

            let api_error: ErrorInformation =
                serde_json::from_str(&text).unwrap_or_else(|_| ErrorInformation {
                    error_information: ApiError {
                        error: 0,
                        message: format!("Unknown error ({}: {})", status, text),
                        code: ApiErrorCode::Unknown,
                    },
                });
            return Err(Error::ApiError(
                api_error.error_information.code,
                api_error.error_information.message,
            ));
        }

        let body: T = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::SerializationError(format!(
                    "Could not deserialize response from \"{}\" ({}).",
                    text,
                    err.to_string()
                )))
            }
        };
        Ok(body)
    }

    async fn put<'a, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let mut delay = INITIAL_DELAY_MS;
        for _ in 0..RETRIES {
            let res = match self.put_without_retry(url, body).await {
                Ok(res) => res,
                Err(err) => {
                    delay = self.randomized_exponential_backoff(delay).await;

                    match err {
                        Error::Throttling => {
                            continue;
                        }
                        _ => return Err(err),
                    };
                }
            };

            return Ok(res);
        }

        Err(Error::Throttling)
    }

    async fn put_without_retry<'a, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &impl Serialize,
    ) -> Result<T, Error> {
        let res = match self.client.put(url).json(body).send().await {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::NetworkError(format!(
                    "Could not send request ({}).",
                    err.to_string()
                )))
            }
        };

        let status = res.status().as_u16();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| String::from("Could not retrieve body text."));

        if status < 200 || status > 299 {
            if status == 429 {
                return Err(Error::Throttling);
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ErrorInformation {
                pub error_information: ApiError,
            }

            #[derive(Deserialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "PascalCase")]
            struct ApiError {
                pub error: u32,
                pub message: String,
                pub code: ApiErrorCode,
            }

            let api_error: ErrorInformation =
                serde_json::from_str(&text).unwrap_or_else(|_| ErrorInformation {
                    error_information: ApiError {
                        error: 0,
                        message: format!("Unknown error ({}: {})", status, text),
                        code: ApiErrorCode::Unknown,
                    },
                });
            return Err(Error::ApiError(
                api_error.error_information.code,
                api_error.error_information.message,
            ));
        }

        let body: T = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(err) => {
                return Err(Error::SerializationError(format!(
                    "Could not deserialize response from \"{}\" ({}).",
                    text,
                    err.to_string()
                )))
            }
        };
        Ok(body)
    }

    // async fn delete<'a, T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
    //     let mut delay = INITIAL_DELAY_MS;
    //     for _ in 0..RETRIES {
    //         let res = match self.delete_without_retry(url).await {
    //             Ok(res) => res,
    //             Err(err) => {
    //                 delay = self.randomized_exponential_backoff(delay).await;

    //                 match err {
    //                     Error::Throttling => {
    //                         continue;
    //                     }
    //                     _ => return Err(err),
    //                 };
    //             }
    //         };

    //         return Ok(res);
    //     }

    //     Err(Error::Throttling)
    // }

    // async fn delete_without_retry<'a, T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
    //     let res = match self.client.delete(url).send().await {
    //         Ok(r) => r,
    //         Err(err) => {
    //             return Err(Error::NetworkError(format!(
    //                 "Could not send request ({}).",
    //                 err.to_string()
    //             )))
    //         }
    //     };

    //     let status = res.status().as_u16();
    //     let text = res
    //         .text()
    //         .await
    //         .unwrap_or_else(|_| String::from("Could not retrieve body text."));

    //     if status < 200 || status > 299 {
    //         if status == 429 {
    //             return Err(Error::Throttling);
    //         }

    //         #[derive(Deserialize, Debug, Clone, PartialEq)]
    //         #[serde(rename_all = "PascalCase")]
    //         struct ErrorInformation {
    //             pub error_information: ApiError,
    //         }

    //         #[derive(Deserialize, Debug, Clone, PartialEq)]
    //         #[serde(rename_all = "PascalCase")]
    //         struct ApiError {
    //             pub error: u32,
    //             pub message: String,
    //             pub code: ApiErrorCode,
    //         }

    //         let api_error: ErrorInformation =
    //             serde_json::from_str(&text).unwrap_or_else(|_| ErrorInformation {
    //                 error_information: ApiError {
    //                     error: 0,
    //                     message: format!("Unknown error ({}: {})", status, text),
    //                     code: ApiErrorCode::Unknown,
    //                 },
    //             });
    //         return Err(Error::ApiError(
    //             api_error.error_information.code,
    //             api_error.error_information.message,
    //         ));
    //     }

    //     let body: T = match serde_json::from_str(&text) {
    //         Ok(r) => r,
    //         Err(err) => {
    //             return Err(Error::SerializationError(format!(
    //                 "Could not deserialize response from \"{}\" ({}).",
    //                 text,
    //                 err.to_string()
    //             )))
    //         }
    //     };
    //     Ok(body)
    // }

    // Randomized exponential backoff policy (cf.
    // https://cloud.google.com/appengine/articles/scalability#backoff ).
    async fn randomized_exponential_backoff(&self, mut delay_ms: f64) -> f64 {
        //let mut rng = rand::thread_rng();

        // Random component to avoid thundering herd problem (values taken from
        // https://github.com/GoogleCloudPlatform/appengine-gcs-client/blob/master/java/src/main/
        // java/com/google/appengine/tools/cloudstorage/RetryHelper.java ).
        //delay_ms = (rng.gen::<f64>() / 2.0 + 0.75) * delay_ms;

        sleep(Duration::from_millis(delay_ms as u64)).await;

        delay_ms *= BACKOFF;
        delay_ms
    }
}
