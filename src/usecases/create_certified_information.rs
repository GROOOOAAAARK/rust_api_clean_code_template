extern crate chrono;
extern crate mockall;
extern crate serde;
extern crate serde_json;

use chrono::{DateTime, Utc, ParseResult, FixedOffset};
use mockall::*;
use mockall::predicate::*;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

use crate::domain::certified_information::CertifiedInformation;
use crate::usecases::ports::response::{Response, ResponseStatus, ResponseMessage};
use crate::usecases::ports::traits::UsecaseTrait;

#[derive(Serialize, Deserialize)]
pub struct CreateCertifiedInformationInput {
    issuance: String,
    data: Map<String, Value>,
    signature: String
}

impl CreateCertifiedInformationInput {
    pub fn new(issuance: String, data: Map<String, Value>, signature: String) -> CreateCertifiedInformationInput {
        CreateCertifiedInformationInput {
            issuance,
            data,
            signature
        }
    }
}

pub struct CreateCertifiedInformationUsecase {}

impl CreateCertifiedInformationUsecase {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
impl UsecaseTrait<CreateCertifiedInformationInput> for CreateCertifiedInformationUsecase {
    fn execute(&self, input: CreateCertifiedInformationInput) -> Response {
        let _input: CreateCertifiedInformationInput = input;

        let issuance_date: ParseResult<DateTime<FixedOffset>> = DateTime::parse_from_rfc3339(&_input.issuance);

        if issuance_date.is_err() {
            println!("Error parsing issuance date: {:?}", issuance_date.err().unwrap().kind()); //TODO: better logging
            return Response::failed(Some(ResponseStatus::BadRequest), Some(ResponseMessage::InvalidIssuanceDateFormat), None);
        }

        let data = _input.data;

        let certified_information: CertifiedInformation = CertifiedInformation::new(
            issuance_date.unwrap().with_timezone(&Utc),
            data,
            _input.signature,
        );

        Response::succeeded(
            Some(ResponseStatus::Created),
            Some(ResponseMessage::CertifiedInformationCreated),
            Some(certified_information.to_json())
        )
    }
}