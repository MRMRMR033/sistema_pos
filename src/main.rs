use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env::{self};

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
    // watch_posts();
    //publish_post::update(); funcionando
    user_controllers::create_user(connection, "Admin", "", "mr4303997@gmail.com", "8991151213", "8994508599", false, true);
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