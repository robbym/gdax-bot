use rayon::prelude::*;
use chrono::prelude::*;
use reqwest::{self, StatusCode};
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Status(reqwest::StatusCode)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::Request(error)
    }
}

pub trait APIEndpoint where for<'a> Self: serde::Deserialize<'a> {
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

pub trait APIEndpoint1 where for<'a> Self: serde::Deserialize<'a> {
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

pub trait APIEndpoint2 where for<'a> Self: serde::Deserialize<'a> {
    fn url(param1: &str, param2: &str) -> String;
    fn get_data(param1: &str, param2: &str) -> Result<Self, Error> {
        let mut resp = reqwest::get(&Self::url(param1, param2))?;
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

pub trait APIEndpoint3 where for<'a> Self: serde::Deserialize<'a> {
    fn url(param1: &str, param2: &str, param3: &str) -> String;
    fn get_data(param1: &str, param2: &str, param3: &str) -> Result<Self, Error> {
        let mut resp = reqwest::get(&Self::url(param1, param2, param3))?;
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

pub trait APIEndpoint4 where for<'a> Self: serde::Deserialize<'a> {
    fn url(param1: &str, param2: &str, param3: &str, param4: &str) -> String;
    fn get_data(param1: &str, param2: &str, param3: &str, param4: &str) -> Result<Self, Error> {
        let mut resp = reqwest::get(&Self::url(param1, param2, param3, param4))?;
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
pub struct ProductData {
    pub id: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub base_min_size: String,
    pub base_max_size: String,
    pub quote_increment: String,
    pub display_name: String,
    pub status: String,
    pub margin_enabled: bool,
    pub status_message: Option<String>,
    pub min_market_funds: String,
    pub max_market_funds: String,
    pub post_only: bool,
    pub limit_only: bool,
}

impl APIEndpoint for Vec<ProductData> {
    fn url() -> String {
        format!("http://api.gdax.com/products")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub min_size: String,
    pub status: String,
    pub message: Option<String>
}

impl APIEndpoint for Vec<Currency> {
    fn url() -> String {
        format!("http://api.gdax.com/currencies")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalData(pub Vec<Vec<f64>>);

impl APIEndpoint1 for HistoricalData {
    fn url(param: &str) -> String {
        format!("http://api.gdax.com/products/{}/candles", param)
    }
}

impl APIEndpoint4 for HistoricalData {
    fn url(param1: &str, param2: &str, param3: &str, param4: &str) -> String {
        format!("http://api.gdax.com/products/{}/candles?start={}&end={}&granularity={}", param1, param2, param3, param4)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    iso: String,
    epoch: f64
}

impl APIEndpoint for Time {
    fn url() -> String {
        format!("http://api.gdax.com/time")
    }
}