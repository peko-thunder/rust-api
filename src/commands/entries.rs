use clap::Parser;
use chrono::Local;
use webapi_mvp::db;
use db::model::*;

#[derive(Debug, PartialEq, Parser)]
struct Args {
    #[arg(short = 'n', long = "project-name", help = "プロジェクト名")]
    proj_name: String,

    #[arg(short = 'u', long = "project-url", help = "プロジェクトのURL文字列")]
    proj_url_name: String,

    #[arg(short = 'N', long = "technology-name", help = "技術名")]
    tech_name: String,

    #[arg(short = 'U', long = "technology-url", help = "技術のURL文字列")]
    tech_url_name: String,
    
    #[arg(short = 'I', long = "technology-image", help = "技術ロゴのURL")]
    tech_image_url: String
}

fn main() {
    let args = Args::parse();
    let conn = db::connection::create();

    let project_record = Project::insert_record(&conn, &NewProject {
        name: &args.proj_name,
        url_name: &args.proj_url_name,
    }).unwrap();
    let technology_record = Technology::insert_record(&conn, &NewTechnology {
        name: &args.tech_name,
        url_name: &args.tech_url_name,
        image_url: &args.tech_image_url,
    }).unwrap();
    Adopt::insert_record(&conn, &NewAdopt {
        projects_id: &project_record.id,
        technologies_id: &technology_record.id,
        created_at: &Local::now().naive_local()
    }).unwrap();
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn commands_entries() {

        let proj_name = "CLIコマンド";
        let proj_url_name = "cli_command";
        let tech_name = "Clap";
        let tech_url_name = "clap";
        let tech_image_url = "http://example.com";

        let args = Args::try_parse_from([
            "",  // コマンド自体を指しているため空文字で良い
            "--project-name", proj_name, 
            "--project-url", proj_url_name, 
            "--technology-name", tech_name, 
            "--technology-url", tech_url_name, 
            "--technology-image", tech_image_url
        ]).unwrap();

        assert_eq!(args, Args {
            proj_name: String::from("CLIコマンド"),
            proj_url_name: String::from("cli_command"),
            tech_name: String::from("Clap"),
            tech_url_name: String::from("clap"),
            tech_image_url: String::from("http://example.com")
        });
    }
}
