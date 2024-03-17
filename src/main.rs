use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::NewUser;
use user::user_controllers;
use std::env;

pub mod publish_post;
pub mod schema;
pub mod models;
pub mod login;
pub mod general_tools;
pub mod products_controllers;
pub mod user;

pub fn establish_connection()-> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_|panic!("Error connecting to {}", database_url))
}

fn main() {
    
    let mut conn = establish_connection();
    //==================================================================================================================
    let usurio = NewUser{
        name: Some("admin"),
        active: Some(&true),
        pass: Some("12345"),
        phone: Some("2030405060"),
        email: None,
        username: Some("admin"),
        lastname: Some("Ramos")

    };

    user_controllers::db_create_user(&mut conn, usurio);


}

