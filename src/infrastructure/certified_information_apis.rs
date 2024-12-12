extern crate actix_web;

use actix_web::{web::Json, web::ServiceConfig, HttpResponse, post};
use crate::adapters::create_certified_information_controller::CreateCertifiedInformationController;
use crate::adapters::http_presenter::HttpPresenter;
use crate::adapters::ports::traits::AdapterTrait;
use crate::usecases::create_certified_information::{CreateCertifiedInformationUsecase, CreateCertifiedInformationInput};

// ** Config **

pub fn routes(service_config: &mut ServiceConfig) {
    service_config.service(create_certified_information);
}

// ** Routes **

#[post("")]
async fn create_certified_information(
    request: Json<CreateCertifiedInformationInput>
) -> HttpResponse {
    let adapter = CreateCertifiedInformationController::new(
        CreateCertifiedInformationUsecase::new(),
        HttpPresenter::new(),
    );
    let input: CreateCertifiedInformationInput = request.into_inner();
    let response = adapter.invoke(input);
    response
}
