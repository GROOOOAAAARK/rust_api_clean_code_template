extern crate actix_web;

use actix_web::HttpResponse;
use mockall::automock;

use crate::adapters::ports::traits::{AdapterTrait, PresenterTrait};
use crate::usecases::create_certified_information::CreateCertifiedInformationInput;
use crate::usecases::ports::traits::UsecaseTrait;

pub struct CreateCertifiedInformationController {
    usecase: Box<dyn UsecaseTrait<CreateCertifiedInformationInput>>,
    presenter: Box<dyn PresenterTrait<HttpResponse>>,
}

impl CreateCertifiedInformationController {
    pub fn new(
        usecase: impl UsecaseTrait<CreateCertifiedInformationInput> + 'static,
        presenter: impl PresenterTrait<HttpResponse> + 'static,
    ) -> Self {
        Self {
            usecase: Box::new(usecase),
            presenter: Box::new(presenter),
        }
    }
}

#[automock]
impl AdapterTrait<CreateCertifiedInformationInput, HttpResponse> for CreateCertifiedInformationController {
    fn invoke(&self, input: CreateCertifiedInformationInput) -> HttpResponse {
        let response = self.usecase.execute(input);
        let http_response: HttpResponse = self.presenter.present(response);
        http_response
    }
}