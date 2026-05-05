use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

pub fn internal_server_error_response(error: String) -> Response<BoxBody<Bytes, hyper::Error>> {
    default_response(error, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_found_error_response(error: String) -> Response<BoxBody<Bytes, hyper::Error>> {
    default_response(error, StatusCode::NOT_FOUND)
}

fn default_response(error: String, status_code: StatusCode) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(status_code)
        .body(
            Full::<Bytes>::new(Bytes::from(error))
                .map_err(|ignore| match ignore {})
                .boxed(),
        )
        .unwrap()
}
