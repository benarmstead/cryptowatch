use binance::api::Binance;
use binance::market::Market;
use std::{env, process, thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut trade_codes: Vec<String> = vec![];

    if args.len() == 1 {
        println!("Must enter at least 1 argument!");
        process::exit(1);
    }

    for i in 1..args.len() {
        trade_codes.push(args[i].to_string())
    }

    let market: Market = Binance::new(None, None);
    loop_print_price(market, trade_codes);
}

fn loop_print_price(market: Market, trade_codes: Vec<String>) {
    let mut price: f64;
    let mut prices: Vec<String> = vec![];

    loop {
        for i in &trade_codes {
            price = get_price(&market, &i.replace("/", ""));
            prices.push(price.to_string());
        }

        clearscreen::clear().expect("failed to clear screen");

        for (x, i) in trade_codes.iter().enumerate() {
            if prices[x] == (-0.1).to_string() {
                println!("{}: {}", i, "Could not find trade code");
            } else {
                println!("{}: {}", i, prices[x]);
            }
        }

        let final_length = prices.len().saturating_sub(trade_codes.len());
        prices.truncate(final_length);

        sleep(3);
    }
}

fn get_price(market: &Market, trade_code: &str) -> f64 {
    match market.get_price(trade_code) {
        Ok(answer) => answer.price,
        Err(_) => -0.1,
    }
}

pub fn sleep(timer: u64) {
    thread::sleep(time::Duration::from_secs(timer));
}

fn _get_data(market: Market) {
    match market.get_24h_price_stats("XMRUSDT") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}
