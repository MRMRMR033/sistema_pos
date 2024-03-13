use std::env::args;
use std::fmt::Error;

use crate::schema::users;
use crate::models::NewUser;
use crate::models::User;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn db_create_user(conn: &mut PgConnection, name: &str, lastname: &str, pass: &str, email:&str, phone: &str,username:&str, active:bool )-> User{  
    let new_user = NewUser {name: &name, active: &active, pass: &pass, phone: &phone, email: &email, username: &username, lastname: &lastname};
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error al aguardar el usuario")
}


/*
    estamos creando una funcion llamada update, donde recibimos como primer parametro una conexion conn.
    segundo parametro el id que deseamos buscar.
    tercer parametro ele
*/
pub fn db_update_user(conn: &mut PgConnection, id_finder:i32,  new_name: Option<&str>, status:bool){
    use crate::schema::users::dsl::*;


    let result = diesel::update(users.filter(id.eq(id_finder)))
        //.set(active.eq(status))
        //si queremos actualizar un unico campo es asi pero si queremos actualizar multiples campos, debemos utilizar una tupla.
        .set((active.eq(status), name.eq(new_name.unwrap_or("default").to_string())))
        .execute(conn);
    println!("{:?}",result)
}

pub fn db_get_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<User> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .ok();
    
    user
}