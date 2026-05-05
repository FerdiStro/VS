use crate::default_responses::{internal_server_error_response, not_found_error_response};
use crate::module_bindings::ApplicationTableAccess;
use crate::{models, AppState};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use spacetimedb_sdk::Identity;
use std::str::FromStr;
use std::sync::Arc;

//get application by id
pub fn get_application(
    request: Request<Incoming>,
    state: &Arc<AppState>,
) -> Response<BoxBody<Bytes, hyper::Error>> {
    let raw_query = request.uri().query().unwrap_or("");

    let parameters =
        match serde_urlencoded::from_str::<models::GetApplicationQueryParameters>(raw_query) {
            Ok(params) => params,
            Err(e) => return internal_server_error_response(e.to_string()),
        };

    let application_id = match parameters.application_id {
        Some(id) => id,
        None => return not_found_error_response("No applicationId provided".to_string()),
    };

    let application_id = match Identity::from_str(&application_id) {
        Ok(id) => id,
        Err(e) => return internal_server_error_response(e.to_string()),
    };

    let res_application = match state
        .db_connection
        .db
        .application()
        .identity()
        .find(&application_id)
    {
        Some(application) => application,
        None => return not_found_error_response("No application found".to_string()),
    };

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
