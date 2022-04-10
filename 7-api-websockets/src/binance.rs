use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct BinanceClient {
    http_client: reqwest::Client,
}

impl BinanceClient {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();
        
        BinanceClient {
            http_client: client
        }
    }

    pub async fn get_exchange_information(&self) -> Result<responses::ExchangeInformationResponse, BinanceError> {
        let request = self.http_client.get("https://api.binance.com/api/v3/exchangeInfo")
            .send()
            .await;

        match request {
            Err(_) => Err(BinanceError::HTTPError),
            Ok(response) => {
                let data = response
                    .json::<responses::ExchangeInformationResponse>()
                    .await;

                if let Ok(x) = data {
                    Ok(x)
                } else {
                    Err(BinanceError::SerializationError)
                }
            }
        }
    }

    pub async fn get_ticket_updates(&self) -> bool {
        let (ws_stream, _) = connect_async("wss://stream.binance.com:9443/ws/!miniTicker@arr")
            .await
            .expect("Failed to conenct");
         
        let (read, write) = ws_stream.split();


        let ws_to_stdout = {
            // read.for_each(|msg| async {
            //     let data = msg.unwrap().into_data();
            //     tokio::io::stdout().write_all(&data).await.unwrap();
            // })
        };

        pin_mut!(ws_to_stdout);

        true
    }
}

pub enum BinanceListener {
    TickerUpdate(String, f64)
}

#[derive(thiserror::Error, Debug)]
pub enum BinanceError {
    #[error("HTTP Client error")]
    HTTPError,
    #[error("Serialization error")]
    SerializationError,
    #[error("Authentication error")]
    AuthenticationError
}

mod responses {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct RateLimit {

    }

    #[derive(Deserialize, Debug)]
    struct Symbol {
        symbol: String,
        baseAsset: String,
        baseAssetPrecision: u8,
        quoteAsset: String,
        quoteAssetPrecision: u8
    }

    #[derive(Deserialize, Debug)]
    pub struct ExchangeInformationResponse {
        timezone: String,
        serverTime: u64,
        rateLimits: Vec<RateLimit>,
        symbols: Vec<Symbol>
    }
}