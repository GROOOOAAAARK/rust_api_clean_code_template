extern crate chrono;
extern crate serde_json;
extern crate strum_macros;

use serde_json::{Map, Value};
use chrono::{DateTime, Utc};

pub struct CertifiedInformation {
    issuance: DateTime<Utc>,
    data: Map<String, Value>,
    signature: String,
}

impl CertifiedInformation {
    pub fn new(issuance: DateTime<Utc>, data: Map<String, Value>, signature: String) -> CertifiedInformation {
        CertifiedInformation{
            issuance,
            data,
            signature,
        }
    }

    pub fn get_issuance(&self) -> DateTime<Utc> {
        self.issuance
    }

    pub fn get_data(&self) -> Map<String, Value> {
        self.data.clone()
    }

    pub fn get_signature(&self) -> String {
        self.signature.clone()
    }
    }
}
