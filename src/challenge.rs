use crate::schema::challenges;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
pub struct Challenge {
    pub id: i32,
    pub name: String,
}
