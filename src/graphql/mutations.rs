use crate::graphql::Context;
use crate::models::*;
use diesel::prelude::*;

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_user(
        context: &Context,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        phone_number: String,
    ) -> User {
        use crate::schema::users;
        let conn = &context.db_con;

        let new_user = NewUser {
            first_name,
            last_name,
            email,
            password,
            phone_number,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new post")
    }
}
