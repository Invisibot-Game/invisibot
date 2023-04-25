use rocket::http::Status;
use rocket::Response;
use rocket::{response::Responder, serde::json::Json};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GameResponse<T: Serialize> {
    data: Option<T>,
    success: bool,
    error: Option<String>,
}

impl<T: Serialize> GameResponse<T> {
    pub fn ok(data: T) -> Self {
        GameResponse {
            data: Some(data),
            success: true,
            error: None,
        }
    }

    pub fn err(err: String) -> Self {
        GameResponse {
            data: None,
            success: false,
            error: Some(err),
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for GameResponse<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> Result<Response<'static>, Status> {
        let json_data = Json(self);
        json_data.respond_to(request)
    }
}
