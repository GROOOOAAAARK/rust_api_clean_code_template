extern crate serde_json;
extern crate strum_macros;

use serde_json::{Map, Value};
use std::option::Option;

use crate::usecases::ports::traits::{Opt};


#[derive(strum_macros::Display, Clone, Debug, PartialEq)]
pub enum ResponseStatus {
    Ok,
    Created,
    Accepted,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    DontUseTeapotForCoffeePot,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
}

impl Opt<ResponseStatus> for Option<ResponseStatus> {
    fn unwrap_or(self, default: ResponseStatus) -> ResponseStatus {
        match self {
            Some(value) => value,
            None => default
        }
    }
}

#[derive(strum_macros::Display, Clone, Debug, PartialEq)]
pub enum ResponseMessage {
    Pong,
    TeaPot,
    CertifiedInformationCreated,
}

impl Opt<ResponseMessage> for Option<ResponseMessage> {
    fn unwrap_or(self, default: ResponseMessage) -> ResponseMessage {
        match self {
            Some(value) => value,
            None => default
        }
    }
}

pub struct Response {
    success: bool,
    status: ResponseStatus,
    message: Option<ResponseMessage>,
    data: Option<Map<String, Value>>
}

impl Response {
    pub fn new(success: bool, status: ResponseStatus, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            success,
            status,
            message,
            data
        }
    }

    pub fn succeeded(status: Option<ResponseStatus>, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            success: true,
            status: status.unwrap_or(ResponseStatus::Ok),
            message: message,
            data: data,
        }
    }

    pub fn failed(status: Option<ResponseStatus>, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            success: false,
            status: status.unwrap_or(ResponseStatus::DontUseTeapotForCoffeePot),
            message,
            data,
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn status(&self) -> ResponseStatus {
        self.status.clone()
    }

    pub fn message(&self) -> Option<ResponseMessage> {
        self.message.clone()
    }

    pub fn data(&self) -> Option<Map<String, Value>> {
        self.data.clone()
    }
}
