//! Use this module to interact with Bitstamp exchange.
//! Please see examples for more informations.

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header::ContentType;
use hyper::net::HttpsConnector;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use helpers;
use bitstamp::tools;

header! {
    #[doc(hidden)]
    (KeyHeader, "Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "Sign") => [String]
}

header! {
    #[doc(hidden)]
    (ContentHeader, "Content-Type") => [String]
}


pub struct BitstampApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    customer_id: String,
    http_client: Client,
}


impl BitstampApi {
    /// Create a new BitstampApi by providing an API key & API secret
    pub fn new(customer_id: &str, api_key: &str, api_secret: &str) -> BitstampApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        BitstampApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            customer_id: customer_id.to_string(),
            http_client: Client::with_connector(connector),
        }
    }

    /// Create a new BitstampApi from a json configuration file. This file must follow this
    /// structure:
    ///
    /// ```ignore
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_bitstamp": {
    ///         "exchange"   : "bitstamp",
    ///         "api_key"    : "1234567890ABCDEF1234567890ABCDEF",
    ///         "api_secret" : "1234567890ABCDEF1234567890ABCDEF",
    ///         "customer_id": "123456"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Bitstamp account with
    /// `new_from_file("account_bitstamp", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> BitstampApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();
        let customer_id = json_obj.get("customer_id").unwrap().as_str().unwrap();

        BitstampApi::new(customer_id, api_key, api_secret)
    }

    fn public_query(&mut self,
                    params: &HashMap<&str, &str>)
                    -> Option<Map<String, Value>> {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);

        let method: &str = params.get("method").unwrap();
        let pair: &str = params.get("pair").unwrap();
        let url: String = tools::build_url(method, pair);

        tools::block_or_continue(self.last_request);
        let mut response = self.http_client.get(&url).send().unwrap();
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return tools::deserialize_json(buffer);
    }

    ///
    ///
    /// #Examples
    ///
    /// ```ignore
    /// extern crate fastcoin;
    /// use fastcoin::bitstamp::BitstampApi;
    /// let mut api = BitstampApi::new("", "");
    /// let  result = api.private_query("balance", "btcusd");
    /// assert_eq!(true, true);
    /// ```
    fn private_query(&mut self,
                     params: &HashMap<&str, &str>)
                     -> Option<Map<String, Value>> {
        let method: &str = params.get("method").unwrap();
        let pair: &str = params.get("pair").unwrap();
        let url: String = tools::build_url(method, pair);

        let nonce = tools::generate_nonce(None);
        let signature = tools::build_signature(nonce.clone(),
                                                     self.customer_id.clone(),
                                                     self.api_key.clone(),
                                                     self.api_secret.clone());

        let copy_api_key = self.api_key.clone();
        let mut post_params: &mut HashMap<&str, &str> = &mut HashMap::new();
        post_params.insert("key", &copy_api_key);
        post_params.insert("signature", &signature);
        post_params.insert("nonce", &nonce);
        helpers::strip_empties(&mut post_params);
        let post_data = helpers::url_encode_hashmap(&post_params);
        let mut response = self.http_client.post(&url)
                            .header(ContentType::form_url_encoded())
                            .body(&post_data)
                            .send()
                            .unwrap();

        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        tools::deserialize_json(buffer)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {
    /// "BTC_LTC":{
    /// "last":"0.0251","lowestAsk":"0.02589999","highestBid":"0.0251",
    /// "percentChange":"0.02390438","baseVolume":"6.16485315","quoteVolume":"245.82513926"},
    /// "BTC_NXT":{
    /// "last":"0.00005730","lowestAsk":"0.00005710","highestBid":"0.00004903",
    /// "percentChange":"0.16701570","baseVolume":"0.45347489","quoteVolume":"9094"},
    /// ... }
    /// ```
    pub fn return_ticker(&mut self) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "ticker");
        params.insert("pair", "btcusd");
        self.public_query(&params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "timestamp": "1234567890"}
    /// ```
    pub fn return_order_book(&mut self,
                             pair: &str)
                             -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "order_book");
        params.insert("pair", pair);
        self.public_query(&params)
    }

    /// Returns all of your available balances.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"BTC":"0.59098578","LTC":"3.31117268", ... }
    /// ```
    pub fn return_balances(&mut self, pair: &str) -> Option<Map<String, Value>> {
        let mut params = HashMap::new();
        params.insert("method", "balance");
        params.insert("pair", pair);
        self.private_query(&params)
    }
}
