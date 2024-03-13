use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl};

use crate::models::User;

pub fn login_init(conn: &mut PgConnection, username: &str)-> Option<User>{
    use crate::schema::users::dsl::*;
    let user = users
        .filter(name.eq(username))
        .first::<User>(conn)
        .ok();

    user
}