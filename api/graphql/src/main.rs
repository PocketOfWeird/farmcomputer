mod conversions;
mod neo;
mod schema;
#[cfg(test)]
mod tests;
mod types;

use conversions::body_to_graphql_request;
use neo::init_context;
use now_lambda::{error::NowError, lambda, IntoResponse, Request};
use schema::init_schema;
use serde_json;
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let graphql_request = body_to_graphql_request(request.into_body());
    println!("graphql_request {:?}", graphql_request);
    let schema = init_schema();

    let grapql_response = graphql_request.execute(&schema, init_context());

    return Ok(serde_json::to_string(&grapql_response).expect("Failed to serialize JSON"));
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
