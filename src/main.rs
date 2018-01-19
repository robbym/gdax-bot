extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;
extern crate rayon;
extern crate csv;

mod api;
use api::*;

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

fn main() {
}
