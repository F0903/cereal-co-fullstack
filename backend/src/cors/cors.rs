use crate::auth::AUTH_COOKIE_NAME;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct CORS {
    allowed_origin: String,
}

impl Default for CORS {
    fn default() -> Self {
        Self {
            allowed_origin: "*".into(),
        }
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

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let authed = {
            let mut response_cookies = response.cookies();
            response_cookies.any(|x| x.name() == AUTH_COOKIE_NAME)
        };

        let origin = {
            let request_headers = _request.headers();
            request_headers.get_one("Origin")
        };

        if authed {
            // If we are authed (at this point the JWT request guard will have verified it),
            // and this is a cross origin request, set the allowed origin to the Origin header value
            if let Some(origin_value) = origin {
                response.set_header(Header::new("Access-Control-Allow-Origin", origin_value));
            }
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        } else {
            response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                self.allowed_origin.clone(),
            ));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}
