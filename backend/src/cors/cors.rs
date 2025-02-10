use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct CORS {}

impl Default for CORS {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let (origin, cors_requested_headers) = {
            let request_headers = request.headers();
            (
                request_headers.get_one("Origin"),
                request_headers.get_one("Access-Control-Request-Headers"),
            )
        };

        // This should always be the case for CORS requests.
        if let Some(origin) = origin {
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == rocket::http::Method::Options {
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, PUT, DELETE, HEAD, GET, OPTIONS",
            ));

            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                cors_requested_headers.unwrap_or("*"),
            ));
            response.set_status(rocket::http::Status::NoContent);
        }
    }
}
