use rocket_dyn_templates::{context, Template};
use thiserror::Error;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use rocket::http::{CookieJar, Cookie};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use chrono::{NaiveDateTime, Utc, Duration};

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        &context! {

        },
    )
}
