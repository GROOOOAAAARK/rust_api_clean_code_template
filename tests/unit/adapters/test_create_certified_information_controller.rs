extern crate actix_web;

use actix_web::HttpResponse;
use actix_web::http::StatusCode;

use rust_clean_code_api::adapters::create_certified_information_controller::{CreateCertifiedInformationController};
use rust_clean_code_api::adapters::http_presenter::{HttpPresenter};
use rust_clean_code_api::adapters::ports::traits::AdapterTrait;
use rust_clean_code_api::usecases::create_certified_information::{MockCreateCertifiedInformationUsecase, CreateCertifiedInformationInput};
use rust_clean_code_api::usecases::ports::response::{Response, ResponseStatus, ResponseMessage};

#[test]
fn test_coherent_created_response() {
    let mut usecase_mock = MockCreateCertifiedInformationUsecase::new();
    let input: CreateCertifiedInformationInput = CreateCertifiedInformationInput::new("2023-01-01T00:00:00Z".to_string(), "".to_string(), "test_signature".to_string());
    usecase_mock.expect_execute().returning(|input| {
        Response::succeeded(Some(ResponseStatus::Created), Some(ResponseMessage::CertifiedInformationCreated), Some(serde_json::from_str(&r#"{"test_data": "test_data"}"#).unwrap()))
    });
    let presenter = HttpPresenter {};
    let controller = CreateCertifiedInformationController::new(usecase_mock, presenter);
    let result: HttpResponse = controller.invoke(input);
    assert_eq!(result.status(), StatusCode::CREATED);
}
