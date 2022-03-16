#[cfg(test)]
mod fastcoin_tests {
    extern crate fastcoin;

    use self::fastcoin::fastcoin::Fastcoin;
    use self::fastcoin::exchange::{ Exchange, ExchangeApi };
    use self::fastcoin::pair::Pair;

    #[test]
    fn can_create_new_api_connection_to_bitstamp() {
        let api: Box<ExchangeApi> = Fastcoin::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");

        assert_eq!(format!("{:?}", api), "BitstampApi { \
                                                last_request: 0, \
                                                api_key: \"bs_api_key\", \
                                                api_secret: \"bs_api_secret\", \
                                                customer_id: \"bs_cust_id\", \
                                                http_client: Client { \
                                                    redirect_policy: FollowAll, \
                                                    read_timeout: None, \
                                                    write_timeout: None, \
                                                    proxy: None \
                                                } \
                                            }");
    }
    #[test]
    fn can_create_new_api_connection_to_kraken() {
//        let api = Fastcoin::new(Exchange::Kraken, "", "", "");
//        assert_eq!(api, Exchange::Kraken);
    }
    #[test]
    fn can_create_new_api_connection_to_poloniex() {
//        let api = Fastcoin::new(Exchange::Poloniex, "", "", "");
//        assert_eq!(api, Exchange::Poloniex);
    }

    #[test]
    fn fastcoin_can_get_a_ticker_from_bitstamp() {
        let mut api = Fastcoin::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let ticker = api.return_ticker(Pair::BtcUsd);

        assert!( ticker.is_some() );
    }
    #[test]
    fn fastcoin_ticker_from_bitstamp_should_have_the_correct_last() {
        let mut api = Fastcoin::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let ticker = api.return_ticker(Pair::BtcUsd);

        assert!( ticker.unwrap().contains_key("last") );
    }

    #[test]
    fn fastcoin_should_return_an_order_book_from_bitstamp() {
        let mut api = Fastcoin::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let order_book = api.return_order_book(Pair::BtcUsd);
        assert!( order_book.is_some() );
    }
    #[test]
    fn order_book_should_have_bids() {
        let mut api = Fastcoin::new(Exchange::Bitstamp, "bs_cust_id", "bs_api_key", "bs_api_secret");
        let result = api.return_order_book(Pair::BtcUsd);
        assert!( result.unwrap().contains_key("bids") );
    }
}
