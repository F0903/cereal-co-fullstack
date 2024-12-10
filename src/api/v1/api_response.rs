use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::Serialize;

pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<ErrorObject>>;

impl<T> From<ApiResponse<T>> for ApiResult<T> {
    fn from(value: ApiResponse<T>) -> Self {
        Self::Ok(value)
    }
}

#[derive(Serialize)]
pub struct ErrorObject {
    error: String,
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    json: Json<T>,
    status: Status,
}

impl ApiResponse<ErrorObject> {
    fn err(message: impl Into<String>, status: Status) -> Self {
        Self {
            json: Json(ErrorObject {
                error: message.into(),
            }),
            status,
        }
    }

    pub fn bad_request() -> Self {
        Self::err("bad request", Status::BadRequest)
    }

    pub fn internal_error() -> Self {
        Self::err("internal server error", Status::InternalServerError)
    }
}

impl<T> ApiResponse<T> {
    pub fn ok(val: T) -> Self {
        Self {
            json: Json(val),
            status: Status::Ok,
        }
    }
}

impl<'r, 'o: 'r, T: serde::Serialize> Responder<'r, 'o> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'o> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
