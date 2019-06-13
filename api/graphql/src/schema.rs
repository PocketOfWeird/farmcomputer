use crate::neo::{call_one, Context};
use crate::types::*;
use juniper::{FieldResult, RootNode};
use uuid::Uuid;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn plant(_context: &Context, id: Uuid) -> FieldResult<Plant> {
        return call_one(&Plant::get_one(id), Plant::mapper);
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn add_plant(_context: &Context, name: String, genus: String) -> FieldResult<Plant> {
        return call_one(&Plant::add_one(name, genus), Plant::mapper);
    }
}

type Schema = RootNode<'static, Query, Mutation>;

pub fn init_schema() -> Schema {
    return Schema::new(Query, Mutation);
}
