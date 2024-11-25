extern crate chrono;
extern crate serde_json;
extern crate strum_macros;

use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct CertifiedInformation<'a> {
    issuance: DateTime<Utc>,
    data: HashMap<&'a str, &'a str>,
    signature: &'a str,
}

impl<'a> CertifiedInformation<'a> {
    pub fn new(issuance: DateTime<Utc>, data: HashMap<&'a str, &'a str>, signature: &'a str) -> CertifiedInformation<'a> {
        CertifiedInformation{
            issuance,
            data,
            signature,
        }
    }

    pub fn get_issuance(&self) -> DateTime<Utc> {
        self.issuance
    }

    pub fn get_data(&self) -> HashMap<&str, &str> {
        self.data.clone()
    }

    pub fn get_signature(&self) -> &str {
        self.signature
    }
}
