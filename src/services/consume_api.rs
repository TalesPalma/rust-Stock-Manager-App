use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

const API_KEY: &str = "b54bcf4d-1bca-4e8e-9a24-22ff2c3d462c"; // API key for accessing CoinMarketCap (test environment)
const BASE_URL: &str = "sandbox-api.coinmarketcap.com"; // Base URL for CoinMarketCap's Sandbox environment
const PARAMETROS_URL: &str = "v1/cryptocurrency/listings/latest"; // Endpoint for retrieving the latest cryptocurrency listings from CoinMarketCap

// Import necessary traits for serializing and deserializing JSON data.
#[derive(Serialize, Deserialize)]
struct DataApi {
    data: Vec<CryptoData>, // A vector of CryptoData objects representing the API response.
}

// Represents individual cryptocurrency data.
#[derive(Serialize, Deserialize)]
struct CryptoData {
    id: i32,        // The unique identifier for the cryptocurrency.
    symbol: String, // The symbol or ticker of the cryptocurrency (e.g., BTC).
    name: String,   // The full name of the cryptocurrency (e.g., Bitcoin).
    quote: Quote,   // Contains the current price of the cryptocurrency in different currencies.
}

// Represents the quote (price) of a cryptocurrency in different currencies.
#[derive(Serialize, Deserialize)]
struct Quote {
    #[serde(rename = "USD")]
    usd: Coin, // Price of the cryptocurrency in USD.
    #[serde(rename = "BTC")]
    btc: Coin, // Price of the cryptocurrency in BTC.
}

// Represents the price of the cryptocurrency in a specific currency.
#[derive(Serialize, Deserialize)]
struct Coin {
    price: f32, // The price of the cryptocurrency in the currency.
}

pub async fn get_price_coin() -> Result<String, Box<dyn std::error::Error>> {
    // Format the URL with base URL and endpoint.
    let url_format = format!("https://{}/{}", BASE_URL, PARAMETROS_URL);

    // Create a new HeaderMap to store the request headers.
    let mut headers = HeaderMap::new();

    // Insert the API key into the headers.
    headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_static(API_KEY));

    let client = reqwest::Client::new();

    // Create a new reqwest HTTP client.
    let resp = client
        .get(url_format)
        .headers(headers)
        .query(&[("start", 1)]) // Add query parameter to the request.
        .send()
        .await?;

    let mut converted: DataApi = DataApi { data: vec![] };
    // Check if the response status indicates success.
    if resp.status().is_success() {
        // Await and get the response body as a string.
        let body = resp.text().await?;
        // Deserialize the JSON response into DataApi struct.
        converted = serde_json::from_str(&body)?;
        // Print the price of the cryptocurrency in BTC from the first item in the data vector.
        println!("{}", converted.data[0].quote.btc.price);
    } else {
        // Print error status and body if the request was not successful.
        println!("Error: {}", resp.status());
        println!("Error: {}", resp.text().await?);
    }

    // Return Ok to indicate that the function completed successfully.
    Ok(converted.data[0].quote.btc.price.to_string())
}
