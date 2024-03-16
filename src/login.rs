use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl};

use crate::models::User;

pub fn login_init(conn: &mut PgConnection, username_input: &str)-> Option<User>{
    use crate::schema::users::dsl::*;
    let user = users
        .filter(username.eq(username_input))
        .first::<User>(conn)
        .ok();

    user
}