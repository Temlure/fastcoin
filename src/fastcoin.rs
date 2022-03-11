use exchange::Exchange;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Fastcoin {
    BitstampApi
}

impl Fastcoin {
    /// Create a new FastcoinApi by providing an API key & API secret
    pub fn new(exchange: Exchange, customer_id: &str, api_key: &str, api_secret: &str) -> Exchange {
        match exchange {
            Exchange::Bitstamp => Exchange::Bitstamp,
            Exchange::Kraken => Exchange::Kraken,
            Exchange::Poloniex => Exchange::Poloniex,
        }
    }
}
