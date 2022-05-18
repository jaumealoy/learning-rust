use std::borrow::BorrowMut;

use tokio_tungstenite::{connect_async, tungstenite::{protocol::Message, handshake::server::Callback}};
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

type CallbackFunction = dyn Fn(&str, f64) -> ();

pub struct BinanceClient {
    http_client: reqwest::Client,
    ticker_listeners: Vec<Box<CallbackFunction>>
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

                let data = serde_json::from_str::<Vec<responses::TickerUpdate>>(&raw_data);

                if let Ok(updates) = data {
                    for symbol in updates {
                        for listener in &self.ticker_listeners {
                //          (*listener)(&symbol.symbol, symbol.get_last_price());
                            listener(&symbol.symbol, symbol.get_last_price());
                        }
                    }
                }
            })
        };

        //pin_mut!(ws_to_stdout);
        ws_to_stdout.await;

        true
    }

    pub fn add_listener(&mut self, callback: Box<CallbackFunction>) {
        self.ticker_listeners
            .push(callback)
    }

    pub async fn get_symbol_price(&self, symbol: &str, time: DateTime<Utc>) -> Option<f64> {
        let end_time = time + Duration::minutes(1);

        let arguments = HashMap::from([
            ("symbol".to_owned(), symbol.to_owned()),
            ("interval".to_owned(), "1m".to_owned()),
            ("startTime".to_owned(), format!("{}", time.timestamp_millis())),
            ("endTime".to_owned(), format!("{}", end_time.timestamp_millis())),
            ("limit".to_owned(), "1".to_owned())
        ]);

        let mut qs: String = String::new();
        for entry in arguments {
            qs.push_str(&format!("{}={}&", entry.0, entry.1));
        }

        let response = self.http_client.get(format!("https://api.binance.com/api/v3/klines?{}", qs))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        match response {
            serde_json::Value::Array(x) => {
                let y = x[0].as_array().unwrap();

                let close = y[4].as_str().unwrap().parse::<f64>();
                if let Ok(price) = close {
                    Some(price)
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

unsafe impl Sync for BinanceClient {}
unsafe impl Send for BinanceClient {}

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
    pub struct Symbol {
        pub symbol: String,
        pub baseAsset: String,
        baseAssetPrecision: u8,
        pub quoteAsset: String,
        quoteAssetPrecision: u8
    }

    #[derive(Deserialize, Debug)]
    pub struct ExchangeInformationResponse {
        timezone: String,
        serverTime: u64,
        rateLimits: Vec<RateLimit>,
        pub symbols: Vec<Symbol>
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