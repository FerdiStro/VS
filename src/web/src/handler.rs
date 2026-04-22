pub mod application_handler;

use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Frame, Incoming};
use hyper::{Request, Response, StatusCode};


//pub fn get_applications() -> Response<BoxBody<Bytes, hyper::Error>> {
  //  DbConnection
//}

//Return "Hello World" on GET
pub fn test_hello(_: Request<Incoming>) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::new(
        Full::<Bytes>::new(Bytes::from("Hello, World!"))
            .map_err(|ignore| match ignore {})
            .boxed(),
    )
}

//Returns PUT body to uppercase ASCII output
pub async fn test_echo_await(request: Request<Incoming>) -> Response<BoxBody<Bytes, hyper::Error>> {
    //buffer max 64kb
    let upper = request.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        return message_response("Only 64 kb payload allowed", StatusCode::PAYLOAD_TOO_LARGE);
    }
    let whole_body = request.collect().await.unwrap().to_bytes();

    let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();

    Response::new(full(reversed_body))
}

//Returns POST body to uppercase ASCII output
pub fn test_echo_strem(request: Request<Incoming>) -> Response<BoxBody<Bytes, hyper::Error>> {
    let frame_stream = request.into_body().map_frame(|frame| {
        let frame_bytes = if let Ok(data) = frame.into_data() {
            data.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Bytes>()
        } else {
            "ERROR".bytes().collect::<Bytes>()
        };
        Frame::data(frame_bytes)
    });
    Response::new(frame_stream.boxed())
}

pub fn message_response(
    message: &str,
    status_code: StatusCode,
) -> Response<BoxBody<Bytes, hyper::Error>> {
    let mut response = Response::new(full(message.to_owned()));
    *response.status_mut() = status_code;
    response
}

pub fn empty_response(status_code: StatusCode) -> Response<BoxBody<Bytes, hyper::Error>> {
    let mut response = Response::new(
        Empty::<Bytes>::new()
            .map_err(|never| match never {})
            .boxed(),
    );
    *response.status_mut() = status_code;
    response
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
