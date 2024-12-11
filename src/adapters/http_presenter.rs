extern crate actix_web;
extern crate serde;
extern crate serde_json;

use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

use crate::adapters::ports::traits::{PresenterTrait, ResponseConverter};
use crate::usecases::ports::response::{Response, ResponseStatus};


#[derive(Serialize, Deserialize)]
struct ResponseBody {
    message: String,
    data: Option<Map<String, Value>>
}

pub struct HttpPresenter {}

impl HttpPresenter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ResponseConverter<HttpResponse> for HttpPresenter {
    fn from_internal_response(&self, response: Response) -> HttpResponse {
        let status: ResponseStatus = response.status();
        let mut response_builder: HttpResponseBuilder = match status {
            ResponseStatus::Ok => HttpResponse::Ok(),
            ResponseStatus::Created => HttpResponse::Created(),
            ResponseStatus::Accepted => HttpResponse::Accepted(),
            ResponseStatus::BadRequest => HttpResponse::BadRequest(),
            ResponseStatus::Unauthorized => HttpResponse::Unauthorized(),
            ResponseStatus::Forbidden => HttpResponse::Forbidden(),
            ResponseStatus::NotFound => HttpResponse::NotFound(),
            ResponseStatus::DontUseTeapotForCoffeePot => HttpResponse::ImATeapot(),
            ResponseStatus::InternalServerError => HttpResponse::InternalServerError(),
            ResponseStatus::NotImplemented => HttpResponse::NotImplemented(),
            ResponseStatus::BadGateway => HttpResponse::BadGateway(),
            ResponseStatus::ServiceUnavailable => HttpResponse::ServiceUnavailable(),
            ResponseStatus::GatewayTimeout => HttpResponse::GatewayTimeout(),
        };

        let https_response: HttpResponse;
        if let Some(data) = response.data() {
            let response_body: ResponseBody = ResponseBody {
                message: response.message().unwrap().to_string(),
                data: Some(data)
            };
            https_response = response_builder.json(response_body);
        }
        else {
            let response_body: ResponseBody = ResponseBody {
                message: response.message().unwrap().to_string(),
                data: None
            };
            https_response = response_builder.json(response_body);
        }

        https_response
    }
}

impl PresenterTrait<HttpResponse> for HttpPresenter {
    fn present(&self, response: Response) -> HttpResponse {
        let http_response = self.from_internal_response(response);

        http_response
    }
}
