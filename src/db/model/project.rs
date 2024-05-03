use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Serialize, Deserialize};
use crate::db::schema::*;

#[derive(Clone, Debug)]
#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub url_name: String,
}

#[derive(Clone, Debug)]
#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub url_name: &'a str,
}

impl Project {
    pub fn insert_record(
        conn: &PgConnection, 
        record: &NewProject,
    ) -> Result<Project, Error> {
        diesel::insert_into(projects::table)
            .values(record)
            .get_result(conn)
    }

    pub fn insert_records(
        conn: &PgConnection, 
        records: &[NewProject],
    ) -> Result<Vec<Project>, Error> {
        diesel::insert_into(projects::table)
            .values(records)
            .get_results(conn)
    }

    pub fn select_by_id(
        conn: &PgConnection, 
        id: &i64, 
    ) -> Result<Project, Error> {
        projects::table.select(projects::all_columns)
            .filter(projects::id.eq(id))
            .get_result(conn)
    }

    pub fn update_by_id(
        conn: &PgConnection,
        id: &i64, 
        record: &NewProject,
    ) -> Result<Project, Error> {
        diesel::update(projects::table)
            .set((
                projects::name.eq(record.name), 
                projects::url_name.eq(record.url_name)
            ))
            .filter(projects::id.eq(id))
            .get_result(conn)
    }

    pub fn delete_by_id(
        conn: &PgConnection, 
        id: &i64, 
    ) -> Result<Project, Error> {
        diesel::delete(projects::table)
            .filter(projects::id.eq(id))
            .get_result(conn)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use crate::db::connection;

    #[test]
    fn crud() {
        connection::test_transaction(|conn| {
            // insert
            let insert_result = Project::insert_record(conn, &NewProject {
                name: "TestProject",
                url_name: "test_project",
            });
            assert!(insert_result.is_ok());

            let project_id = insert_result.unwrap().id;

            // select
            let select_result = Project::select_by_id(conn, &project_id);
            assert!(select_result.is_ok());

            // update
            let update_result = Project::update_by_id(conn, &project_id, &NewProject {
                name: "NewTestProject",
                url_name: "new_test_project",
            });
            assert!(update_result.is_ok());
            let select_result = Project::select_by_id(conn, &project_id)?;
            assert_eq!(select_result.name, "NewTestProject");
            assert_eq!(select_result.url_name, "new_test_project");

            // delete
            let delete_result = Project::delete_by_id(conn, &project_id);
            assert!(delete_result.is_ok());
            let select_result = Project::select_by_id(conn, &project_id);
            assert!(select_result.is_err());

            Ok(())
        }).unwrap()
    }
}
