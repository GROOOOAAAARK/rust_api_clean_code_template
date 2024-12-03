extern crate serde_json;
extern crate chrono;

use chrono::{DateTime, Utc};

use crate::domain::certified_information::CertifiedInformation;
use crate::usecases::ports::response::{Response, ResponseStatus, ResponseMessage};

pub struct CreateCertifiedInformationInput {
    issuance: String,
    data: String,
    signature: String
}

impl CreateCertifiedInformationInput {
    pub fn new(issuance: String, data: String, signature: String) -> CreateCertifiedInformationInput {
        CreateCertifiedInformationInput {
            issuance,
            data,
            signature
        }
    }
}

pub struct CreateCertifiedInformationUsecase {}

impl CreateCertifiedInformationUsecase {
    pub fn execute(&self, input: CreateCertifiedInformationInput) -> Response {
        let _input: CreateCertifiedInformationInput = input;

        let certified_information: CertifiedInformation = CertifiedInformation::new(
            DateTime::parse_from_rfc3339(&_input.issuance).unwrap().with_timezone(&Utc),
            serde_json::from_str(&_input.data).unwrap(),
            _input.signature,
        );

        Response::succeeded(
            Some(ResponseStatus::Created),
            Some(ResponseMessage::CertifiedInformationCreated),
            Some(certified_information.to_json())
        )
    }
}