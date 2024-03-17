use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewProduct, NewUser};
use user::user_controllers;
use std::{env, time::SystemTime};

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
    let time = SystemTime::now();
    let producto = NewProduct{
        name: "Shampoo palmolive",
        bar_code: "7501",
        cost_price: &BigDecimal::from_f64(19.00).unwrap(),
        sales_price: &BigDecimal::from_f64(22.00).unwrap(),
        promotion_price: &BigDecimal::from_f64(21.00).unwrap(),
        stock: true,
        updated_at: &time,
    };

    products_controllers::db_update_product(&mut conn, 1, producto)


}

