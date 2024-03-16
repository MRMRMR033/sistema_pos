use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::{models::{NewUser, User}, schema::users};

pub fn db_create_user(conn: &mut PgConnection, name: &str, lastname: &str, pass: &str, email:&str, phone: &str,username:&str, active:bool )-> User{  
    let new_user = NewUser {name: &name, active: &active, pass: &pass, phone: &phone, email: &email, username: &username, lastname: &lastname};
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error al aguardar el usuario")
}