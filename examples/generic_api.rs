// This example shows how to use the generic API provided by Fastcoin.
// This method is useful if you have to iterate throught multiple accounts of
// different exchanges and perform the same operation (such as get the current account's balance)
// You can also use the Fastcoin generic API if you want a better error handling since all methods
// return Result<_, Error>.

extern crate fastcoin;

use fastcoin::fastcoin::Fastcoin;
use fastcoin::exchange::Exchange::*;
use fastcoin::pair::Pair::*;

fn main() {
    // We create a Fastcoin Generic API
    // Since Kraken does not need customer_id field, we set it to None
    let mut my_api = Fastcoin::new(Kraken, "api_key", "api_secret", None);
    let ticker = my_api.ticker(ETC_BTC);

    println!("ETC_BTC last trade price is {}.",
             ticker.unwrap().last_trade_price);
}
