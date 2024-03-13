use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use user_controllers::get_user_by_id;
use std::{env, io::{self, stdin, BufRead, Stdin}};

pub mod publish_post;
use crate::models::Post;
use self::models::NewPost;
pub mod schema;
pub mod models;
pub mod user_controllers;

pub fn establish_connection()-> PgConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_|panic!("Error connecting to {}", database_url))
}

pub fn watch_posts(){
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loadin posts");
    println!("Displaying {}", results.len());

    for post in results {
        println!("{}", post.title);
        println!("---------------");
        println!("{}", post.body);
    }
}
fn main() {
    let connection = &mut establish_connection();
    // create_posts(connection, "eres una puta", "Ok no te");
    watch_posts();
    //publish_post::update(); funcionando
    //user_controllers::create_user(connection, "admin", true);
    //user_controllers::update_user(connection, 2,Some("Prueba"), false);



    let mut find_user_input = String::new();

    println!("Ingrese el ID del usuario:");
    io::stdin().lock().read_line(&mut find_user_input).expect("Error al recibir el parámetro");
    
    // Eliminar el salto de línea al final y convertir a i32
    let find_user_id: i32 = match find_user_input.trim().parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Error: Entrada no válida para el ID de usuario.");
            return;
        }
    };

    let user = get_user_by_id(connection, find_user_id);
    let user_un = user.unwrap();
    println!("{:?}", user_un);


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

