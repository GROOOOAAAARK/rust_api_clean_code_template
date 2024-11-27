use chrono::{TimeZone, Utc, DateTime};
use serde_json::{Map, Value};
use rust_clean_code_api::domain::certified_information::CertifiedInformation;

use super::*;

#[test]
fn test_new_and_getters() {
    let issuance: DateTime<Utc> = Utc.ymd(2023, 1, 1).and_hms(0, 0, 0);
    let mut data: Map<String, Value> = Map::new();
    data.insert("key".to_string(), Value::String("value".to_string()));
    let signature: String = "test_signature".to_string();

    let cert_info = CertifiedInformation::new(issuance, data.clone(), signature.clone());

    assert_eq!(cert_info.get_issuance(), issuance);
    assert_eq!(cert_info.get_data(), data);
    assert_eq!(cert_info.get_signature(), signature.clone());
}
}
