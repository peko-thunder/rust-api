use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::db::model::*;
use crate::db::schema::*;

pub fn get_technology_page_by_url_name(
  conn: &PgConnection, 
  url_name: &str
) -> Result<(Technology, Vec<Project>), Error> {
  let tech = Technology::select_by_url_name(conn, url_name)?;
  let projs = get_projects_by_technologies_id(&conn, tech.id)?;

  Ok((tech, projs))
}

fn get_projects_by_technologies_id(
  conn: &PgConnection, 
  tech_id: i64
) -> Result<Vec<Project>, Error> {
  projects::table
      .inner_join(adopts::table.on(projects::id.eq(adopts::projects_id)))
      .filter(adopts::technologies_id.eq(tech_id))
      .select(projects::all_columns)
      .load(conn)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod unit_DBテスト {
  use super::*;
  use crate::db::connection;

  #[test]
  fn get_technology_page_by_url_name関数はTechnologyページの情報をDBから一括で取得する() {
      connection::test_transaction(|conn| {
          let new_project = NewProject {
              name: "TestProject",
              url_name: "test_project",
          };
          let new_technology = NewTechnology {
              name: "TestTechnology",
              url_name: "test_technology",
              image_url: "https://example.com/",
          };

          let project_record = Project::insert_record(&conn, &new_project)?;
          let technology_record = Technology::insert_record(&conn, &new_technology)?;
          Adopt::insert_record(conn, &NewAdopt {
              projects_id: &project_record.id,
              technologies_id: &technology_record.id,
              created_at: &chrono::NaiveDate::from_ymd_opt(2016, 7, 8)
                  .unwrap().and_hms_opt(9, 10, 11).unwrap()
          })?;

          let (tech, projs) = get_technology_page_by_url_name(&conn, new_technology.url_name)?;
          let proj = &projs[0];
          assert_eq!(tech.name, new_technology.name);
          assert_eq!(tech.image_url, new_technology.image_url);
          assert_eq!(proj.name, new_project.name);
          assert_eq!(proj.url_name, new_project.url_name);
  
          Ok(())
      }).unwrap(); 
  }
}
