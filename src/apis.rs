use actix_web::{HttpServer, App, get, web, HttpResponse, Responder};
use tera::{Tera, Context};
use crate::db;
use crate::db::model::{Technology, Project};

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_technologiy_page)
    })
    .bind((addr, port))?
    .run()
    .await
}

#[get("/technologies/{tech_name}")]
async fn get_technologiy_page(tech_name: web::Path<String>) -> impl Responder {
    // DBへアクセスして技術ページに表示する情報を取得する
    let conn = db::connection::create();
    let path = tech_name.to_string();
    let (tech, projs) = match db::interface::get_technology_page_by_url_name(&conn, &path) {
        Ok(result) => result,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    //HTML形式のレスポンスボディを生成する
    match render_technology_page(tech, projs) {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

fn render_technology_page(
    tech: Technology, 
    projs: Vec<Project>
) -> Result<String, tera::Error> {
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("tech", &tech);
    ctx.insert("projs", &projs);

    tmpl.render("tech_page.html", &ctx) 
}
