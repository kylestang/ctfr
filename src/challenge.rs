use crate::schema::challenges;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
pub struct Challenge {
    id: i32,
    name: String,
}
