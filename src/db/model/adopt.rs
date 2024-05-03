use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db::schema::*;

#[derive(Clone, Debug)]
#[derive(Queryable)]
pub struct Adopt {
    pub id: i64,
    pub projects_id: i64,
    pub technologies_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name = "adopts"]
pub struct NewAdopt<'a> {
    pub projects_id: &'a i64,
    pub technologies_id: &'a i64,
    pub created_at: &'a chrono::NaiveDateTime,
    // TODO: Noneの場合は自動で現在時刻を設定できるようにしたい
    // pub created_at: &'a Option<chrono::NaiveDateTime>,
}

impl Adopt {
    pub fn insert_record(
        conn: &PgConnection, 
        record: &NewAdopt,
    ) -> Result<Adopt, Error> {
        diesel::insert_into(adopts::table)
            .values(record)
            .get_result(conn)
    }

    pub fn insert_records(
        conn: &PgConnection, 
        records: &[NewAdopt],
    ) -> Result<Vec<Adopt>, Error> {
        diesel::insert_into(adopts::table)
            .values(records)
            .get_results(conn)
    }
}
