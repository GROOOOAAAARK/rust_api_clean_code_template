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

#[test]
fn test_from_json() {
    const ISSUANCE: &str = "2023-01-01T00:00:00Z";
    const DATA: &str = "{\"key\":\"value\"}";
    const SIGNATURE: &str = "test_signature";
    let mut json = Map::new();
    json.insert("issuance".to_string(), Value::String(ISSUANCE.to_string()));
    json.insert("data".to_string(), Value::String(DATA.to_string()));
    json.insert("signature".to_string(), Value::String(SIGNATURE.to_string()));

    let cert_info = CertifiedInformation::from_json(json);

    assert_eq!(cert_info.get_issuance(), DateTime::parse_from_rfc3339(ISSUANCE).unwrap().with_timezone(&Utc));
    let mut expected_data = Map::new();
    expected_data.insert("key".to_string(), Value::String("value".to_string()));
    assert_eq!(cert_info.get_data(), expected_data);
    assert_eq!(cert_info.get_signature().clone(), SIGNATURE);
}
}
