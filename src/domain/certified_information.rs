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

    pub fn from_json(json: Map<String, Value>) -> CertifiedInformation {
        let issuance_str: &str = json.get("issuance").unwrap().as_str().unwrap();
        let issuance: DateTime<Utc> = DateTime::parse_from_rfc3339(issuance_str).unwrap().with_timezone(&Utc);
        let data_str: &str = json.get("data").unwrap().as_str().unwrap();
        let data: Map<String, Value> = serde_json::from_str(data_str).unwrap();
        let signature: String = json.get("signature").unwrap().to_string();
        CertifiedInformation{
            issuance,
            data,
            signature,
        }
    }
    }
}
