use crate::{db::DbCon, db::PgPool};

use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};

pub mod mutations;
pub mod queries;

pub struct Context{
   pub db_con: DbCon,
}

impl juniper::Context for Context {}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Context, ()> {
        let db_pool = request.guard::<State<PgPool>>()?;

        match db_pool.get() {
            Ok(db_con) => Outcome::Success(Context { db_con }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
