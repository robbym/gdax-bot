extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;
extern crate rayon;
extern crate csv;

use rayon::prelude::*;
use chrono::prelude::*;
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

trait APIEndpoint2 where for<'a> Self: serde::Deserialize<'a> {
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

trait APIEndpoint3 where for<'a> Self: serde::Deserialize<'a> {
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

trait APIEndpoint4 where for<'a> Self: serde::Deserialize<'a> {
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

impl APIEndpoint4 for HistoricalData {
    fn url(param1: &str, param2: &str, param3: &str, param4: &str) -> String {
        format!("http://api.gdax.com/products/{}/candles?start={}&end={}&granularity={}", param1, param2, param3, param4)
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

fn market_data(duration: chrono::Duration) -> Vec<Vec<f64>> {
    let endtime = chrono::Utc::now();
    let mut starttime = endtime.clone().checked_sub_signed(duration).unwrap();
    let mut times = vec![];
    while starttime <  endtime {
        let next = starttime.clone().checked_add_signed(chrono::Duration::minutes(350)).unwrap();
        times.push((starttime, next));
        starttime = next;
    }

    times.iter().flat_map(|&(start, end)| {
        std::thread::sleep(std::time::Duration::from_millis(450));
        <HistoricalData as APIEndpoint4>::get_data("BTC-USD",
            &start.to_rfc3339().replace("+00:00", ""),
            &end.to_rfc3339().replace("+00:00", ""),
            "60").unwrap().0.into_iter()
    }).collect()
}

fn first_algorithm() {
    let mut rdr = csv::Reader::from_file("./data-month.csv").unwrap();
    let stream = rdr.decode().into_iter().map(|record| {
        let (time, low, high, open, close, volume): (u32, f32, f32, f32, f32, f32) = record.unwrap();
        low
    });

    let mut money = 1000.0;
    let mut btc = 0.0;

    let mut last_price = 0.0;
    let mut falling = true;
    let mut max = 0.0;
    let mut total = 0.0;

    let mut prices = vec![];

    enum FSM {
        Idle,
        Falling,
        Rising
    }
    let mut state = FSM::Idle;

    let stake = 0.2;
    let min_btc = 0.01;
    let max_btc = 8.0;

    let buf_size = 1;

    let mut count = 0;

    let mut buy_price = 0.0;

    for price in stream {
        let min = price * btc;

        if prices.len() > buf_size {
            prices.push(price);
            prices.remove(0);
        } else {
            prices.push(price);
        }

        let coef = pearson_coef(&prices);

        match state {
            FSM::Idle => {
                if prices.len() > buf_size && price < last_price {
                // if prices.len() > buf_size && coef < 0.0 {
                    state = FSM::Falling;
                }
            }
            FSM::Falling => {
                if price > last_price {
                // if coef > 0.0 {
                    if money >= price * min_btc {
                        if money * stake > price * min_btc {
                            if (money * stake) / price > max_btc {
                                btc += max_btc;
                                money -= price * max_btc;
                                buy_price = price * max_btc;
                            } else {
                                btc += (money * stake) / price;
                                money -= money * stake;
                                buy_price = money * stake;
                            }
                        } else {
                            btc += min_btc;
                            money -= price * min_btc;
                            buy_price = price * min_btc;
                        }
                    }
                    state = FSM::Rising;
                }
            }
            FSM::Rising => {
                if coef < 0.0 && (btc * price) > buy_price {
                    money += btc * price;
                    btc = 0.0;
                    state = FSM::Falling;
                }
            }
        }



        last_price = price;
        total = money + btc * price;
        if total > max {
            max = total;
        }

        if count > 1440 {
            count = 0;
            println!("Money: {}, BTC: {}, Total: {}", money, btc, money + (btc * last_price));
        } else {
            count += 1;
        }
    }
    println!("Money: {}, BTC: {}, Total: {}", money, btc, money + (btc * last_price));
    println!("Max: {}", max);
}

fn pearson_coef(data: &[f32]) -> f32 {
    let mut x_avg = 0.0;
    let mut y_avg = 0.0;
    for x in (0..data.len()) {
        x_avg += x as f32;
        y_avg += data[x];
    }
    x_avg /= data.len() as f32;
    y_avg /= data.len() as f32;

    let mut top = 0.0;
    let mut bot_x = 0.0;
    let mut bot_y = 0.0;
    for x in (0..data.len()) {
        top += (x as f32 - x_avg) * (data[x] - y_avg);
        bot_x += (x as f32 - x_avg) * (x as f32 - x_avg);
        bot_y += (data[x] as f32 - y_avg) * (data[x] as f32 - y_avg);
    }
    top / (bot_x * bot_y).sqrt()
}

fn pull_year() {
    let mut data = market_data(chrono::Duration::weeks(4));
    data.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
    let mut csv = File::create("data-month.csv").unwrap();
    for row in data {
        let s = format!("{}, {}, {}, {}, {}, {}\n", row[0], row[1], row[2], row[3], row[4], row[5]);
        csv.write_all(s.as_bytes()).unwrap();
    }
}

use std::fs::File;
use std::io::Write;
fn main() {
    first_algorithm();
}
