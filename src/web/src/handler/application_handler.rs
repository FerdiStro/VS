use crate::models;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};

pub fn get_application(request: Request<Incoming>) -> Response<BoxBody<Bytes, hyper::Error>> {
    let raw_query = request.uri().query().unwrap_or("");

    match serde_urlencoded::from_str::<models::GetApplicationQueryParameters>(raw_query) {
        Ok(params) => {
            println!("Parsed params - ID: {:?}", params.application_id);
        }
        Err(e) => {
            eprintln!("Failed to parse query: {}", e);
        }
    }
    Response::new(
        Empty::<Bytes>::new()
            .map_err(|never| match never {})
            .boxed(),
    )
}

//create application
pub fn create_application(request: Request<Incoming>) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::new(
        Empty::<Bytes>::new()
            .map_err(|never| match never {})
            .boxed(),
    )
}
