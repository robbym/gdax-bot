extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
use reqwest::StatusCode;

#[derive(Debug)]
enum Error {
    Request(reqwest::Error),
    Status(reqwest::StatusCode)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::Request(error)
    }
}

trait APIEndpoint where for<'a> Self: serde::Deserialize<'a> {
    fn url() -> String;

    fn get_data() -> Result<Self, Error> {
        let mut resp = reqwest::get(&Self::url())?;
        match resp.status() {
            StatusCode::Ok => {
                Ok(serde_json::from_str(&resp.text().unwrap()).unwrap())
            }
            code => {
                Err(Error::Status(code))
            }
        }
    }
}

trait APIEndpoint1 where for<'a> Self: serde::Deserialize<'a> {
    fn url(param: &str) -> String;
    fn get_data(param: &str) -> Result<Self, Error> {
        let mut resp = reqwest::get(&Self::url(param))?;
        match resp.status() {
            StatusCode::Ok => {
                Ok(serde_json::from_str(&resp.text().unwrap()).unwrap())
            }
            code => {
                Err(Error::Status(code))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ProductData {
    id: String,
    base_currency: String,
    quote_currency: String,
    base_min_size: String,
    base_max_size: String,
    quote_increment: String,
    display_name: String,
    status: String,
    margin_enabled: bool,
    status_message: Option<String>,
    min_market_funds: String,
    max_market_funds: String,
    post_only: bool,
    limit_only: bool,
}

impl APIEndpoint for Vec<ProductData> {
    fn url() -> String {
        format!("http://api.gdax.com/products")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    id: String,
    name: String,
    min_size: String,
    status: String,
    message: Option<String>
}

impl APIEndpoint for Vec<Currency> {
    fn url() -> String {
        format!("http://api.gdax.com/currencies")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoricalData(Vec<Vec<f64>>);

impl APIEndpoint1 for HistoricalData {
    fn url(param: &str) -> String {
        format!("http://api.gdax.com/products/{}/candles", param)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Time {
    iso: String,
    epoch: f64
}

impl APIEndpoint for Time {
    fn url() -> String {
        format!("http://api.gdax.com/time")
    }
}

fn market_data() {
    println!("{:#?}", HistoricalData::get_data("BTC-USD"));
}

fn main() {
    market_data();
}
