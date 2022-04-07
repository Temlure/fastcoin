// This example shows how to use the generic API provided by fastcoin
// This method is useful if you have to iterate throught multiple accounts of
// different exchanges and perform the same operation (such as get the current account's balance)

extern crate fastcoin;

use fastcoin::fastcoin::Fastcoin;
use fastcoin::exchange::Exchange::*;
use fastcoin::pair::Pair;

fn main() {
    // We create a Fastcoin Generic API
    // Since Poloniex does not need customer_id field, we keep it empty
    let mut my_api = Fastcoin::new(Poloniex, "", "api_key", "api_secret");
    my_api.return_balances(Pair::ETC_BTC);
}
