use crate::graphql::Context;
use crate::models::*;
use diesel::prelude::*;

pub struct Query;
#[juniper::object(Context = Context)]
impl Query {
    fn users(context: &Context) -> Vec<User> {
        use crate::schema::users;
        let conn = &context.db_con;
        let users = users::table.load::<User>(conn).expect("");
        users
    }
}
