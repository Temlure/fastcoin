#[cfg(test)]
mod fastcoin_tests {
    extern crate fastcoin;

    use self::fastcoin::fastcoin::Fastcoin;
    use self::fastcoin::exchange::{ Exchange, ExchangeApi };

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
}
