#[macro_use] extern crate rocket;

mod binance;
use binance::BinanceClient;

mod market_graph;
use market_graph::MarketGraph;

use std::env;
use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use tokio::join;
use chrono::{DateTime, Utc, NaiveDateTime};

use rocket::serde::json::{Value, json, self};

#[tokio::main]
async fn main() {
    // Initialize Binance Client
    let mut binance_client = BinanceClient::new();

    // fetch available markets
    let exchange_information = binance_client.get_exchange_information()
        .await
        .unwrap();

    let mut initial_graph = MarketGraph::new();

    for market in exchange_information.symbols {
        if !initial_graph.has_currency(&market.baseAsset) {
            initial_graph.add_currency(&market.baseAsset)
        }

        if !initial_graph.has_currency(&market.quoteAsset) {
            initial_graph.add_currency(&market.quoteAsset)
        }

        initial_graph.create_market(&market.symbol, &market.baseAsset, &market.quoteAsset);
    }

    //let mut graph = RefCell::new(initial_graph);


    let mut graph = Arc::new(RwLock::new(initial_graph));

    // add event listeners
    let my_graph = graph.clone();
    binance_client.add_listener(Box::new(move |market, price| {
        //println!("{} -> {}", market, price);
    
        let mut writer = my_graph.write().unwrap();
        writer.update_price(&String::from(market), price);
    }));

    // connect to websockets client and subscribe to tickers channel
    let is_connected = binance_client.get_ticket_updates();

    // create API
    let mut config = rocket::Config::default();
    
    let port_env = std::env::var("PORT")
        .unwrap_or("8000".to_owned());

    let port = port_env.parse::<u16>().unwrap();
    config.port = port;

    let api = rocket::build()
        .configure(config)
        .manage(graph.clone())
        .manage(Arc::new(BinanceClient::new()))
        .mount("/", routes![world, convert])
        .launch();

    join!(is_connected, api);
}

#[get("/<base>/<quote>")]
fn world(graph: &rocket::State<Arc<RwLock<MarketGraph>>>, base: &str, quote: &str) -> Value {
    let reader = graph.read().unwrap();
    let price = reader.get_price(&base.to_owned(), &quote.to_owned());
    json!(price)
}

#[get("/historic/<base>/<quote>?<time>")]
async fn convert(graph: &rocket::State<Arc<RwLock<MarketGraph>>>, client: &rocket::State<Arc<BinanceClient>>, base: &str, quote: &str, time: i64) -> Value {
    let conversion_path = graph
        .read()
        .unwrap()
        .get_conversion_path(base, quote);
    
    match conversion_path {
        Some(path) => {
            let time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(time / 1000, 0), Utc);

            let mut amount: f64 = 1.0;
            for conversion in path {
                let price = client.get_symbol_price(&conversion.0, time)
                    .await;

                if let Some(value) = price {
                    if conversion.1 { // buy
                        amount = amount / value;
                    } else { // sell
                        amount = amount * value;
                    }
                } else {
                    return Value::Null;
                }
            }
            json!(amount)
        },
        None => Value::Null
    }
}