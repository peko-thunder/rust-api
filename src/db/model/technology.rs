use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Serialize, Deserialize};
use crate::db::schema::*;

#[derive(Clone, Debug)]
#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub url_name: String,
    pub image_url: String,
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name = "technologies"]
pub struct NewTechnology<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
    pub image_url: &'a str,
}

impl Technology {
    pub fn insert_record(
        conn: &PgConnection, 
        record: &NewTechnology,
    ) -> Result<Technology, Error> {
        diesel::insert_into(technologies::table)
            .values(record)
            .get_result(conn)
    }

    pub fn insert_records(
        conn: &PgConnection, 
        records: &[NewTechnology],
    ) -> Result<Vec<Technology>, Error> {
        diesel::insert_into(technologies::table)
            .values(records)
            .get_results(conn)
    }

    pub fn select_by_url_name(
        conn: &PgConnection, 
        url_name: &str
    ) -> Result<Technology, Error> {
        technologies::table
            .filter(technologies::url_name.eq(url_name))
            .get_result(conn)
    }
}
