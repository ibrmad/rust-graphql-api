use crate::db::establish_connection;
use crate::graphql::{mutations::Mutation, queries::Query, Context};
use juniper::RootNode;
use rocket::{get, post, response::content, routes, State};

type Schema = RootNode<'static, Query, Mutation>;

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

pub fn launch() {
    rocket::ignite()
        .manage(establish_connection())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, get_graphql_handler],
        )
        .launch();
}
