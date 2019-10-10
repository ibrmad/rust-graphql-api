use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::GraphQLObject;
#[derive(GraphQLObject, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub phone_number: String,
    #[graphql(skip)]
    pub verification_code: Option<i32>,
    pub is_phone_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}
