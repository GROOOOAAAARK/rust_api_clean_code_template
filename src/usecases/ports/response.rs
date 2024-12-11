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
    InvalidIssuanceDateFormat,
    InvalidDataFormat,
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
    _success: bool,
    _status: ResponseStatus,
    _message: Option<ResponseMessage>,
    _data: Option<Map<String, Value>>
}

impl Response {
    pub fn new(success: bool, status: ResponseStatus, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            _success: success,
            _status: status,
            _message: message,
            _data: data
        }
    }

    pub fn succeeded(status: Option<ResponseStatus>, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            _success: true,
            _status: status.unwrap_or(ResponseStatus::Ok),
            _message: message,
            _data: data,
        }
    }

    pub fn failed(status: Option<ResponseStatus>, message: Option<ResponseMessage>, data: Option<Map<String, Value>>) -> Response{
        Response{
            _success: false,
            _status: status.unwrap_or(ResponseStatus::DontUseTeapotForCoffeePot),
            _message: message,
            _data: data,
        }
    }

    pub fn success(&self) -> bool {
        self._success
    }

    pub fn status(&self) -> ResponseStatus {
        self._status.clone()
    }

    pub fn message(&self) -> Option<ResponseMessage> {
        self._message.clone()
    }

    pub fn data(&self) -> Option<Map<String, Value>> {
        self._data.clone()
    }
}
