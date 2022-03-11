use serde_json::value::Map;
use serde_json::value::Value;

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Exchange {
    Bitstamp,
    Kraken,
    Poloniex,
}

pub trait ExchangeApi: Debug {
    fn public_query(&mut self, params: &HashMap<&str, &str>) -> Option<Map<String, Value>>;
    fn private_query(&mut self, params: &HashMap<&str, &str>) -> Option<Map<String, Value>>;

    fn return_ticker(&mut self) -> Option<Map<String, Value>>;
    fn return_order_book(&mut self, pair: &str) -> Option<Map<String, Value>>;
    fn return_balances(&mut self, pair: &str) -> Option<Map<String, Value>>;
}