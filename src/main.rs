use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use iced::{Application, Settings};
use models::{NewProduct, NewUser};
use std::{env, time::SystemTime};

pub mod views;
pub mod schema;
pub mod models;
pub mod login;
pub mod general_tools;
pub mod controllers;
use views::view_login;

pub fn establish_connection()-> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_|panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut conn = establish_connection();
    view_login::main();
}
