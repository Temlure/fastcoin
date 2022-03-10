#[cfg(test)]
mod fastcoin_tests {
    extern crate fastcoin;

    use self::fastcoin::fastcoin::Fastcoin;
    use self::fastcoin::exchange::Exchange;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api = Fastcoin::new(Exchange::Bitstamp, "", "", "");
        assert_eq!(api, Exchange::Bitstamp);
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
        let api = Fastcoin::new(Exchange::Kraken, "", "", "");
        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
        let api = Fastcoin::new(Exchange::Poloniex, "", "", "");
        assert_eq!(api, Exchange::Poloniex);
    }
}
