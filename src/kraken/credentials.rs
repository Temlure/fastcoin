//! Contains the Kraken credentials.

use std::collections::HashMap;
use std::str::FromStr;

use serde_json;
use serde_json::Value;

use crate::fastcoin::Credentials;
use crate::exchange::Exchange;
use crate::helpers;
use crate::error::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct KrakenCreds {
    exchange: Exchange,
    name: String,
    data: HashMap<String, String>,
}

impl KrakenCreds {
    /// Create a new `KrakenCreds` from arguments.
    pub fn new(name: &str, api_key: &str, api_secret: &str) -> Self {
        let mut creds = KrakenCreds {
            data: HashMap::new(),
            exchange: Exchange::Kraken,
            name: if name.is_empty() {
                "KrakenClient".to_string()
            } else {
                name.to_string()
            },
        };


        //if api_key.is_empty() {
        //warning!("No API key set for the Bitstamp client");
        //}
        creds
            .data
            .insert("api_key".to_string(), api_key.to_string());

        //if api_secret.is_empty() {
        //warning!("No API secret set for the Bitstamp client");
        //}
        creds
            .data
            .insert("api_secret".to_string(), api_secret.to_string());

        creds
    }


    /// Create a new `KrakenCreds` from a json configuration file. This file must follow this
    /// structure:
    ///
    /// ```json
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
    /// For this example, you could use load your Kraken account with
    /// `KrakenAPI::new(KrakenCreds::new_from_file("account_kraken", Path::new("/keys.json")))`
    pub fn new_from_file(name: &str, path: PathBuf) -> Result<Self> {
        let mut f = File::open(&path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let data: Value = serde_json::from_str(&buffer)?;
        let json_obj = data.as_object()
            .ok_or_else(|| ErrorKind::BadParse)?
            .get(name)
            .ok_or_else(|| ErrorKind::MissingField(name.to_string()))?;
        let api_key = helpers::get_json_string(json_obj, "api_key")?;
        let api_secret = helpers::get_json_string(json_obj, "api_secret")?;
        let exchange = {
            let exchange_str = helpers::get_json_string(json_obj, "exchange")?;
            Exchange::from_str(exchange_str)
                .chain_err(|| ErrorKind::InvalidFieldValue("exchange".to_string()))?
        };

        if exchange != Exchange::Kraken {
            return Err(ErrorKind::InvalidConfigType(Exchange::Kraken, exchange).into());
        }

        Ok(KrakenCreds::new(name, api_key, api_secret))
    }
}

impl Credentials for KrakenCreds {
    /// Return a value from the credentials.
    fn get(&self, key: &str) -> Option<String> {
        if let Some(res) = self.data.get(key) {
            Some(res.clone())
        } else {
            None
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn exchange(&self) -> Exchange {
        self.exchange
    }
}
