mod handler;
mod logger;

use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::{body::Bytes, server::conn::http1, Method, Request, Response, StatusCode};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::{Service, ServiceBuilder, ServiceExt};

async fn router(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/test") => Ok(handler::test_hello(req)),
        (&Method::POST, "/test") => Ok(handler::test_echo_strem(req)),
        (&Method::PUT, "/test") => Ok(handler::test_echo_await(req).await),
        _ => Ok(handler::empty_response(StatusCode::NOT_FOUND)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::spawn(async move {
            let svc = tower::service_fn(router);
            let svc = ServiceBuilder::new()
                .layer_fn(logger::Logger::new)
                .service(svc);

            let svc = TowerToHyperService::new(svc);
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("server error: {}", err);
            }
        });
    }
}
