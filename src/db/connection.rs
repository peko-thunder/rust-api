use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn create() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Dieselは非同期に対応していないので、トランザクション内部に別の非同期処理を挟むことができない
/// 非同期を使える候補先としてSeaORMがある
pub fn test_transaction<F, R>(test_fn: F) -> Result<R, Error>
    where F: FnOnce(&PgConnection) -> Result<R, Error>,
{
    let connection = create();
    Ok(connection.test_transaction(|| test_fn(&connection)))
}
