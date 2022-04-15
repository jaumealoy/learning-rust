#[macro_use] extern crate rocket;

mod binance;
use binance::BinanceClient;

mod market_graph;
use market_graph::MarketGraph;

use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use tokio::join;

use rocket::serde::json::{Value, json};

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
        println!("{} -> {}", market, price);
    
        let mut writer = my_graph.write().unwrap();
        writer.update_price(&String::from(market), price);
    }));

    // connect to websockets client and subscribe to tickers channel
    let is_connected = binance_client.get_ticket_updates();

    // create API
    let api = rocket::build()
        .manage(graph.clone())
        .mount("/", routes![world])
        .launch();

    join!(is_connected, api);
}

#[get("/<base>/<quote>")]
fn world(graph: &rocket::State<Arc<RwLock<MarketGraph>>>, base: &str, quote: &str) -> Value {
    let reader = graph.read().unwrap();
    let price = reader.get_price(&base.to_owned(), &quote.to_owned());
    json!(price)
}