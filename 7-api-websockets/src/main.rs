mod binance;
use binance::BinanceClient;

mod market_graph;
use market_graph::MarketGraph;

use std::cell::RefCell;

#[tokio::main]
async fn main() {
    // Initialize Binance Client
    let mut binance_client = BinanceClient::new();

    // fetch available markets
    let exchange_information = binance_client.get_exchange_information()
        .await
        .unwrap();

    // println!("{:?}", exchange_information);
    let mut graph = RefCell::new(MarketGraph::new());

    for market in exchange_information.symbols {
        if !graph.borrow().has_currency(&market.baseAsset) {
            graph.borrow_mut().add_currency(&market.baseAsset)
        }

        if !graph.borrow().has_currency(&market.quoteAsset) {
            graph.borrow_mut().add_currency(&market.quoteAsset)
        }

        graph.borrow_mut().create_market(&market.symbol, &market.baseAsset, &market.quoteAsset);

        //graph.add_neighbour(&symbol.baseAsset, &symbol.quoteAsset);
    }

    // add event listeners
    binance_client.add_listener(Box::new(move |market, price| {
        println!("{} -> {}", market, price);

        graph.borrow_mut()
            .update_price(&String::from(market), price);
    }));

    // connect to websockets client and subscribe to tickers channel
    let is_connected = binance_client.get_ticket_updates()
        .await;

    // TODO: create API
}