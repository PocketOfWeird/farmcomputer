use juniper::http::GraphQLRequest;
use now_lambda::Body;

pub fn body_to_graphql_request(body: Body) -> GraphQLRequest {
    let request = String::from(body);
    return GraphQLRequest::new(request, None, None);
}
