use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError;

#[derive(Serialize)]
pub struct MessageObject {
    pub message: String,
}

#[derive(Serialize)]
pub struct IdObject {
    pub id: i32,
}

#[derive(Debug)]
pub struct ApiResponse<T> {
    json: Json<T>,
    status: Status,
}

impl ApiResponse<ApiError> {
    fn err(status: Status) -> Self {
        Self {
            json: Json(ApiError),
            status,
        }
    }

    pub fn bad_request() -> Self {
        Self::err(Status::BadRequest)
    }

    pub fn internal_error() -> Self {
        Self::err(Status::InternalServerError)
    }

    pub fn unauthorized() -> Self {
        Self::err(Status::Unauthorized)
    }
}

impl ApiResponse<MessageObject> {
    fn message(message: impl Into<String>) -> Self {
        Self {
            json: Json(MessageObject {
                message: message.into(),
            }),
            status: Status::Ok,
        }
    }

    pub fn success() -> Self {
        Self::message("success")
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
        match self.status.code {
            200 /* OK */ => Response::build_from(self.json.respond_to(&req).unwrap())
                .status(self.status)
                .header(ContentType::JSON)
                .ok(),
            _ => Err(self.status),
        }
    }
}
