use serde_json::{Map, Value};
use rust_clean_code_api::usecases::create_certified_information::{CreateCertifiedInformationUsecase, CreateCertifiedInformationInput};
use rust_clean_code_api::usecases::ports::response::{Response, ResponseStatus, ResponseMessage};
use rust_clean_code_api::usecases::ports::traits::UsecaseTrait;

#[test]
fn test_create_right_input() {
    let usecase = CreateCertifiedInformationUsecase {};
    let mut data = Map::new();
    data.insert("key".to_string(), Value::String("value".to_string()));
    let input: CreateCertifiedInformationInput = CreateCertifiedInformationInput::new(
        "2023-01-01T00:00:00Z".to_string(),
        data.clone(),
        "test_signature".to_string(),
    );
    let result: Response = usecase.execute(input);
    assert_eq!(result.success(), true);
    assert_eq!(result.status(), ResponseStatus::Created);
    assert_eq!(result.message().unwrap(), ResponseMessage::CertifiedInformationCreated);
    assert_eq!(result.data().is_some(), true);
    assert_eq!(result.data().unwrap().get("issuance").unwrap(), "2023-01-01T00:00:00Z");
    assert_eq!(result.data().unwrap().get("data").unwrap(), &serde_json::to_value(data).unwrap());
    assert_eq!(result.data().unwrap().get("signature").unwrap(), "test_signature");
}

#[test]
fn test_create_wrong_issuance_date_format() {
    let usecase = CreateCertifiedInformationUsecase {};
    let input: CreateCertifiedInformationInput = CreateCertifiedInformationInput::new("202382-0212341".to_string(), serde_json::from_str(&r#"{"test_data": "test_data"}"#).unwrap(), "test_signature".to_string());
    let result: Response = usecase.execute(input);
    assert_eq!(result.success(), false);
    assert_eq!(result.status(), ResponseStatus::BadRequest);
    assert_eq!(result.message().unwrap(), ResponseMessage::InvalidIssuanceDateFormat);
}
