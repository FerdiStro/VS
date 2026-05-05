mod default_responses;
mod handler;
mod logger;

pub mod models;
pub mod module_bindings;

use crate::handler::application_handler::{create_application, get_application};
use crate::module_bindings::DbConnection;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::{body::Bytes, server::conn::http1, Method, Request, Response, StatusCode};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use spacetimedb_sdk::__codegen::log;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::{Service, ServiceBuilder, ServiceExt};

const DB_NAME: &str = "dazzling-current-2640";
const DB_SERVER_URL: &str = "https://maincloud.spacetimedb.com";

struct AppState {
    db_connection: DbConnection,
}

async fn router(
    req: Request<hyper::body::Incoming>,
    app_state: Arc<AppState>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/test") => Ok(handler::test_hello(req)),
        (&Method::POST, "/test") => Ok(handler::test_echo_strem(req)),
        (&Method::PUT, "/test") => Ok(handler::test_echo_await(req).await),
        (&Method::POST, "/application") => Ok(create_application(req)),
        (&Method::GET, "/application") => Ok(get_application(req, &app_state)),
        (&Method::GET, "/applicationS") => Ok(handler::test_echo_await(req).await),
        _ => Ok(handler::empty_response(StatusCode::NOT_FOUND)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_connection = connect_to_spacetime_db();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    let state = Arc::new(AppState { db_connection });

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let state_clone = state.clone();

        tokio::spawn(async move {
            let svc = tower::service_fn(move |req| router(req, state_clone.clone()));

            let svc = ServiceBuilder::new()
                .layer_fn(logger::Logger::new)
                .service(svc);

            let svc = TowerToHyperService::new(svc);

            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                log::error!("server error: {}", err);
            }
        });
    }
}

fn connect_to_spacetime_db() -> DbConnection {
    DbConnection::builder()
        .with_uri("https://maincloud.spacetimedb.com")
        .with_database_name("dazzling-current-2640")
        .on_connect(|_ctx, _identity, token| {
            println!("Connected!");
        })
        .on_connect_error(|_ctx, error| {
            eprintln!("Connection failed: {}", error);
        })
        .on_disconnect(|_ctx, error| {
            if let Some(err) = error {
                eprintln!("Disconnected with error: {}", err);
            } else {
                println!("Disconnected normally");
            }
        })
        .build()
        .expect("Failed to connect")
}
