use crate::{establish_connection, models::{NewPost, Post}};
use diesel::prelude::*;
use std::env::args;

pub fn update() {
    use crate::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Published post {}", post.title);
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