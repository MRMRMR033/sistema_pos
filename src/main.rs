use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use user::user_controllers::db_get_user_by_id;
use std::{env, io::{self, BufRead}};

pub mod publish_post;
use crate::{general_tools::clear_console, models::Post};
use self::models::NewPost;
pub mod schema;
pub mod models;
pub mod login;
pub mod general_tools;
pub mod products_controllers;
pub mod user;
use login as loginuser;
pub fn establish_connection()-> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_|panic!("Error connecting to {}", database_url))
}

fn main() {
    //==================================================================================================================
    clear_console();
    //en este bloque vamos a desarrollar toda la logica del login
    let connection = &mut establish_connection();
    let usuario;
    'login: loop{
        let mut username = String::new();
        let mut password = String::new();
        println!("Por favor ingrese su nombre de usuario");
        io::stdin().lock().read_line(&mut username).expect("Error al capturar el nombre de usuario");
        clear_console();
        println!("Por favor ingrese su contrasena");
        io::stdin().lock().read_line(&mut password).expect("Error al recibir la contrasena");
        clear_console();
        let user = loginuser::login_init(connection, &username.trim());
        let user_unwrapped = user.unwrap();
        if &user_unwrapped.username == &username.trim() && &user_unwrapped.pass.to_string() == password.trim(){
            usuario = user_unwrapped;
            break 'login;
        }
    }
    println!("{}, {}", &usuario.email, &usuario.active);
    //==================================================================================================================
    let mut find_user_input = String::new();

    println!("Ingrese el ID del usuario:");
    io::stdin().lock().read_line(&mut find_user_input).expect("Error al recibir el parámetro");
    
    let find_user_id: i32 = match find_user_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Error: Entrada no válida para el ID de usuario.");
            return;
        }
    };
    let user = db_get_user_by_id(connection, find_user_id).unwrap();
    println!("{:?}", user);


}

pub fn create_posts(conn: &mut PgConnection, title: &str, body: &str)-> Post{
    use crate::schema::posts;
    let new_post = NewPost {title, body};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
