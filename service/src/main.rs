#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod database;
pub mod schema;
pub mod routes;

use rocket_dyn_templates::Template;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket::{Build, Rocket};
use rocket::serde::Deserialize;

use self::routes::*;
use self::database::TransactionsDb;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = TransactionsDb::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("can run migrations");

    rocket
}

#[derive(Deserialize)]
struct Config {
    statics: Option<String>,
}


#[launch]
fn rocket() -> _ {
    let builder = rocket::build();
    let figment = builder.figment();
    let config: Config = figment.extract().expect("config");

    let statics = config.statics.unwrap_or_else(|| relative!("static").to_owned());
    builder
        .mount("/", FileServer::from(&statics))
        .mount(
            "/",
            routes![
                index,
            ],
        )
        // .manage(Cache::new())
        .attach(Template::fairing())
        .attach(TransactionsDb::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
}
