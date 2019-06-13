fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let graphql_request:
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
