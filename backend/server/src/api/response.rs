use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Clone)]
pub struct GameResponse<T: Serialize + Clone> {
    pub status: Status,
    pub response_data: ResponseData<T>,
    pub headers: HashMap<&'static str, &'static str>,
}

impl<T: Serialize + Clone> GameResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: Status::Ok,
            response_data: ResponseData::Success(data),
            headers: HashMap::new(),
        }
    }

    pub fn ok_with_status(data: T, status: Status) -> Self {
        Self {
            status,
            response_data: ResponseData::Success(data),
            headers: HashMap::new(),
        }
    }

    pub fn err(status: Status, msg: String) -> Self {
        Self {
            status,
            response_data: ResponseData::Failure(msg),
            headers: HashMap::new(),
        }
    }

    pub fn internal_err() -> Self {
        Self {
            status: Status::InternalServerError,
            response_data: ResponseData::Failure("Internal server error".to_string()),
            headers: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize)]
pub enum ResponseData<T: Serialize + Clone> {
    Success(T),
    Failure(String),
}

impl<T: Serialize + Clone> Display for ResponseData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseData::Success(_) => write!(f, "Success"),
            ResponseData::Failure(err_msg) => write!(f, "Error: {err_msg}"),
        }
    }
}

impl<'r, T: Serialize + Clone> Responder<'r, 'static> for GameResponse<T> {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'static>, Status> {
        if self.status.code >= 400 {
            warn!(
                "Status: {} :: ResponseData: {}",
                self.status.code, self.response_data
            )
        } else {
            info!("Success, status: {}", self.status)
        }

        let response_data = match self.response_data {
            ResponseData::Success(data) => json!(data),
            ResponseData::Failure(err_msg) => json!(err_msg),
        };

        let mut response = Response::build_from(response_data.respond_to(request)?);
        response.status(self.status);
        for (name, val) in self.headers.into_iter() {
            response.raw_header(name, val);
        }

        response.ok()
    }
}
