// use webapi_mvp::{apis, db};
// use actix_web::{test, App};
// use db::model::*;
// use diesel::pg::PgConnection;
// use diesel::result::Error;

// fn before_insert(
//     conn: &PgConnection, 
// ) -> Result<(), Error> {
//     let project_record = Project::insert_record(conn, &NewProject {
//         name: "ProjectA",
//         url_name: "project-a",
//     })?;
//     let technology_record = Technology::insert_record(&conn, &NewTechnology {
//         name: "AWS",
//         url_name: "aws",
//         image_url: "https://example.com/",
//     })?;
//     Adopt::insert_record(conn, &NewAdopt {
//         projects_id: &project_record.id,
//         technologies_id: &technology_record.id,
//         created_at: &chrono::NaiveDate::from_ymd_opt(2016, 7, 8)
//             .unwrap().and_hms_opt(9, 10, 11).unwrap()
//     })?;

//     Ok(())
// }

#[cfg(test)]
#[allow(non_snake_case)]
mod integration_api_db {
    // use super::*;
    use webapi_mvp::apis;
    use actix_web::{test, App};
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn get_page_status_200() {
        // before_insert(conn)?;
        // DBにデータがないとテストが通らない
        // cargo run --bin entry -- -n ProjectA -u project-a -N AWS -U aws -I https://a0.awsstatic.com/libra-css/images/logos/aws_smile-header-desktop-en-white_59x35.png

        let path = "/technologies/aws";
        let tech_name = "AWS";

        // テスト用サーバの立ち上げ
        let service = App::new().service(apis::get_technologiy_page);
        let app = test::init_service(service).await;

        // テスト用のリクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // ステータスコードが200であることを確認
        assert_eq!(resp.status(), StatusCode::OK);

        // 取得した値をパースして確認
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains(tech_name));
    }

    #[actix_web::test]
    async fn get_page_status_404() {
        let path = "/technologies/hoge";

        // テスト用サーバの立ち上げ
        let service = App::new().service(webapi_mvp::apis::get_technologiy_page);
        let app = test::init_service(service).await;

        // リクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // ステータスコードが404であることを確認
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
