use crate::schema::users;
use crate::models::NewUser;
use crate::models::User;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn db_create_user(conn: &mut PgConnection, new_user: NewUser)-> User{  
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error al aguardar el usuario")
}
//TODO: cuando vayamos a hacer un update a un usuario, primero debemos obtener todos sus datos con la funcion
// db_get_user_by_id 
pub fn db_update_user(conn: &mut PgConnection, id_finder:i32,  update_user: NewUser){
    use crate::schema::users::dsl::*;
    let result = diesel::update(users.filter(id.eq(id_finder)))
        //.set(active.eq(status))
        //si queremos actualizar un unico campo es asi pero si queremos actualizar multiples campos, debemos utilizar una tupla.
        .set(&update_user)
        .execute(conn);
    println!("Update completado: {:?}", result);
}

pub fn db_get_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<User> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .ok();
    user
}