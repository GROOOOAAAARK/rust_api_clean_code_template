use chrono::{TimeZone, Utc, DateTime};
use std::collections::HashMap;

use rust_clean_code_api::domain::certified_information::CertifiedInformation;

pub mod test_certified_information {
    use super::*;

    #[test]
    fn test_new_and_getters() {
        let issuance: DateTime<Utc> = Utc.ymd(2023, 1, 1).and_hms(0, 0, 0);
        let mut data: HashMap<&str, &str> = HashMap::new();
        data.insert("key", "value");
        let signature: &str = "test_signature";

        let cert_info = CertifiedInformation::new(issuance, data.clone(), signature);

        assert_eq!(cert_info.get_issuance(), issuance);
        assert_eq!(cert_info.get_data(), data);
        assert_eq!(cert_info.get_signature(), signature);
    }
}
