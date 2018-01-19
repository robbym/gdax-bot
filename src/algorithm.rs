
fn first_algorithm() {
    let mut rdr = csv::Reader::from_file("./data-month.csv").unwrap();
    let stream = rdr.decode().into_iter().map(|record| {
        let (time, low, high, open, close, volume): (u32, f32, f32, f32, f32, f32) = record.unwrap();
        low
    });

    let mut money = 1000.0;
    let mut btc = 0.0;
    let mut last_price = 0.0;

    enum FSM {
        Idle,
        Falling,
        Rising
    }
    let mut state = FSM::Idle;

    let stake = 0.2;
    let min_btc = 0.01;
    let max_btc = 8.0;

    let mut buy_price = 0.0;

    for price in stream {
        let min = price * btc;

        match state {
            FSM::Idle => {
                if price < last_price {
                    state = FSM::Falling;
                }
            }
            FSM::Falling => {
                if price > last_price {
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
                if price < last_price && (btc * price) > buy_price {
                    money += btc * price;
                    btc = 0.0;
                    state = FSM::Falling;
                }
            }
        }
        last_price = price;
    }
}