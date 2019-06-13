use super::*;

#[test]
fn handler_handles() {
    let request = Request::new(now_lambda::Body::from(
        "mutation {\n addPlant(name: \"Broccoli\", genus: \"Brassica\"){\nname\ngenus}}",
    ));
    let expected = serde_json::json!({
        "data": {
            "addPlant": {
                "genus": "Brassica",
                "name": "Broccoli",
            }
        }
    })
    .into_response();
    let response = handler(request)
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}
