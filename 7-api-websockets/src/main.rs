mod binance;
use binance::BinanceClient;

#[tokio::main]
async fn main() {
    // Initialize Binance Client
    let binance_client = BinanceClient::new();

    // fetch available markets
    let exchange_information = binance_client.get_exchange_information()
        .await
        .unwrap();

    println!("{:?}", exchange_information);

    // TODO: connect to websockets client
    let is_connected = binance_client.get_ticket_updates()
        .await;

    // TODO: subscribe to tickers channel

    // TODO: add event listeners

    // TODO: create API
}
