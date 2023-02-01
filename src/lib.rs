use reqwest;
use std::error::Error as StdError;
use std::fmt;

pub mod commands;
mod request;
pub mod response;

pub struct WalletClient {
    clt: reqwest::Client,
    endpoints: Endpoints,
    pubkey: String,
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wallet client error: {}", self.desc())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl StdError for Error {}

impl Error {
    pub fn desc(&self) -> String {
        use Error::*;
        match self {
            ReqwestError(e) => format!("reqwest error: {}", e),
        }
    }
}

struct Endpoints {
    pub token_header: String,
    pub base_url: String,
    pub health: String,
    pub request: String,
}

impl Endpoints {
    pub fn new(base_url: &str, token: &str) -> Endpoints {
        return Endpoints {
            token_header: format!("VWT {}", token),
            base_url: base_url.to_string(),
            health: format!("{}/api/v2/health", base_url),
            request: format!("{}/api/v2/requests", base_url),
        };
    }
}

impl WalletClient {
    pub async fn new(
        wallet_address: &str,
        token: &str,
        pubkey: &str,
    ) -> Result<WalletClient, Error> {
        let w = WalletClient {
            clt: reqwest::Client::new(),
            endpoints: Endpoints::new(wallet_address, token),
            pubkey: pubkey.to_string(),
        };

        w.check_health().await?;
        return Ok(w);
    }

    pub async fn check_health(&self) -> Result<(), Error> {
        let _ = self.clt.get(&self.endpoints.health).send().await?;
        return Ok(());
    }

    pub fn send(&self) {}

    pub fn sign(&self) {}

    pub async fn list_keys(&self) -> Result<response::KeysResponse, Error> {
        let resp = self
            .clt
            .post(&self.endpoints.request)
            .json(&request::Request::new_list_keys())
            .header("Origin", &self.endpoints.base_url)
            .header("Authorization", &self.endpoints.token_header)
            .send()
            .await?;
        return Ok(resp
            .json::<response::Response<response::KeysResponse>>()
            .await?
            .result);
    }
}

#[cfg(test)]
mod tests {
    use super::commands::*;

    #[test]
    fn it_works() {
        let command = Command::OrderCancellation(OrderCancellation {
            order_id: "e5d5039ec7acb756a409c4b50fabf1f7923716ce65ddf618e460366a4c3912e4"
                .to_string(),
            market_id: "5d69ff4a485a9f963272c8614c1d0d84bb8ea57886f5b11aad53f0ccc77731ba"
                .to_string(),
        });

        let pegged_order = PeggedOrder {
            reference: PeggedReference::BestAsk,
            offset: "100".to_string(),
        };

        println!("{}", serde_json::to_string(&command).unwrap());
        println!("{}", serde_json::to_string(&pegged_order).unwrap());
    }
}
