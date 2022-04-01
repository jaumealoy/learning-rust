use std::{future::Future, os::windows::raw};
use serde::{Deserialize, Serialize };


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let ip = get_my_ip()
        .await?;

    println!("Your IP is: {}", ip);

    success_response();

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct MyIPResponse {
    success: bool,
    ip: String,
    r#type: String
}

async fn get_my_ip() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get("https://api.my-ip.io/ip.json")
        .send()
        .await?
        .json::<MyIPResponse>()
        .await?;

    Ok(response.ip)
}

#[derive(Deserialize, Debug)]
enum MyResponse {
    SuccessResponse {
        status: bool,
        msg: String
    },
    Error {
        status: bool
    }
}

fn success_response() {
    let raw_response = "{ \"status\": true, \"msg\": \"ok\" }".to_owned();
    
    let data = serde_json::from_str::<MyResponse>(&raw_response).unwrap();
    println!("{:?}", data);

    if let MyResponse::SuccessResponse { status, msg } = data {
        println!("{}", msg);
    } else {
        println!("Failed response");
    }
}