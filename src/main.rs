use binance::api::*;
use binance::market::*;

fn main() {
    let trade = "XMRUSDT".to_string();
    let market: Market = Binance::new(None, None);
    loop_print_price(market, trade);
}

fn loop_print_price(market: Market, trade: String) {
    println!("{}", get_price(market, trade));
}

fn get_price(market: Market, trade: String) -> String {
    match market.get_price(trade) {
        Ok(answer) => return answer.price.to_string(),
        Err(e) => return e.to_string(),
    }
}

fn _get_data(market: Market) {
    match market.get_24h_price_stats("XMRUSDT") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}
