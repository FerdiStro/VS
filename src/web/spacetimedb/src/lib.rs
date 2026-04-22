use spacetimedb::{Identity, ReducerContext, Table};
use std::io::empty;
use std::thread::available_parallelism;

#[derive(spacetimedb::SpacetimeType)]
enum Status {
    OPEN,
    DENIED,
    CALL,
    OFFER,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::OPEN => write!(f, "OPEN"),
            Status::DENIED => write!(f, "DENIED"),
            Status::CALL => write!(f, "CALL"),
            Status::OFFER => write!(f, "OFFER"),
        }
    }
}
#[spacetimedb::table(accessor = application, public)]
pub struct Application {
    #[primary_key]
    identity: Identity,
    name: String,
    status: Status,
    job_description_url: Option<String>,
    addition_url: Option<String>,
}

#[spacetimedb::reducer]
pub fn add(ctx: &ReducerContext, name: String, job_description_url: String, addition_url: String) {
    let application = Application {
        identity: ctx.sender(),
        name,
        status: Status::OPEN,
        job_description_url: Some(job_description_url),
        addition_url: Some(addition_url),
    };

    ctx.db.application().insert(application);
}

#[spacetimedb::reducer]
pub fn update(ctx: &ReducerContext, id: Identity) -> Result<(), String> {
    match validate_name(ctx, id) {
        Ok(mut application) => {
            application.status = Status::CALL;
            ctx.db.application().identity().update(application);
            Ok(())
        }
        Err(err_string) => Err(err_string),
    }
}

#[spacetimedb::reducer]
pub fn display_all(ctx: &ReducerContext) {
    for application in ctx.db.application().iter() {
        log::info!(
            "ID: {}, Application: {}, with status: {}",
            application.identity.to_string(),
            application.name,
            application.status
        );
    }
}

fn validate_name(ctx: &ReducerContext, id: Identity) -> Result<Application, String> {
    ctx.db
        .application()
        .identity()
        .find(id)
        .ok_or_else(|| "No Application".to_string())
}
