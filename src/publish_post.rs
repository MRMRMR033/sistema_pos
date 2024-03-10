use crate::{establish_connection, models::Post};
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