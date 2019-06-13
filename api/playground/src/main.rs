use juniper::http::playground::playground_source;
use now_lambda::{error::NowError, lambda, IntoResponse, Request};
use std::error::Error;

fn handler(_request: Request) -> Result<impl IntoResponse, NowError> {
    return Ok(playground_source(&"/graphql"));
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
