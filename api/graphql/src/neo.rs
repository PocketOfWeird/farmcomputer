use juniper::FieldResult;
use rusted_cypher::cypher::result::Row;
use rusted_cypher::cypher::CypherResult;
use rusted_cypher::error::GraphError;
use rusted_cypher::GraphClient;
use std::env;

pub struct Context;

impl juniper::Context for Context {}

pub fn init_context() -> &'static Context {
    return &Context {};
}

fn new_from_row<T>(mapper: fn(&Row) -> T, row: &Row) -> T {
    return mapper(row);
}

pub fn call<T>(statement: &str, mapper: fn(&Row) -> T) -> FieldResult<Vec<T>> {
    let neo_url = env::var("NEO_URL").expect("Neo url variable not found");
    let graph = GraphClient::connect(neo_url).expect("Unable to connect to the neo4j database");
    let result: CypherResult = graph.exec(statement).or_else(|e: GraphError| Err(e))?;
    let mut objects: Vec<T> = Vec::new();
    for row in result.rows() {
        let object: T = new_from_row(mapper, &row);
        objects.push(object);
    }
    return Ok(objects);
}

pub fn call_one<T>(statement: &str, mapper: fn(&Row) -> T) -> FieldResult<T> {
    let ok = call(statement, mapper);
    let mut field_result_vec = ok.unwrap();
    let object: T = field_result_vec.pop().unwrap();
    return Ok(object);
}
