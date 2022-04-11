use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct BinanceClient {
    http_client: reqwest::Client,
    ticker_listeners: Vec<Box<dyn Fn(&str, f64) -> ()>>
}

impl BinanceClient {
    pub fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .build()
            .unwrap();
        
        BinanceClient {
            http_client: client,
            ticker_listeners: vec![]
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
         
        let (_write, read) = ws_stream.split();


        let ws_to_stdout = {
            read.for_each(|msg| async {
                // let data = msg.unwrap().into_data();
                // tokio::io::stdout().write_all(&data).await.unwrap();

                let raw_data = msg
                    .unwrap()
                    .into_text()
                    .unwrap();

                let data = serde_json::from_str::<Vec<responses::TickerUpdate>>(&raw_data)
                    .unwrap();

                for symbol in data {
                    for listener in &self.ticker_listeners {
                        listener(&symbol.symbol, symbol.get_last_price());
                    }

                    // println!("Symbol {}: {}", symbol.symbol, symbol.get_last_price())
                }
            })
        };

        //pin_mut!(ws_to_stdout);
        ws_to_stdout.await;

        true
    }

    pub fn add_listener(&mut self, callback: Box<dyn Fn(&str, f64) -> ()>) {
        self.ticker_listeners
            .push(callback)
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

    #[derive(Deserialize, Debug)]
    pub struct TickerUpdate {
        #[serde(rename="s")]
        pub symbol: String,
        
        #[serde(rename="c")]
        last_price: String
    }

    impl TickerUpdate {
        pub fn get_last_price(&self) -> f64 {
            self.last_price.parse::<f64>()
                .unwrap()
        }
    }
}