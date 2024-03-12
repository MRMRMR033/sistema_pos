use crate::schema::users;
use crate::models::NewUser;
use crate::models::User;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
pub fn create_user(conn: &mut PgConnection, name: &str, active:bool )-> User{  
    let new_user = NewUser {name: &name, active: &active};
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
pub fn update_user(conn: &mut PgConnection, id_finder:i32,  new_name: &str, status:bool){
    use crate::schema::users::dsl::*;


    let result = diesel::update(users.filter(id.eq(id_finder)))
        //.set(active.eq(status))
        //si queremos actualizar un unico campo es asi pero si queremos actualizar multiples campos, debemos utilizar una tupla.
        .set((active.eq(status), name.eq(new_name)))
        .execute(conn);
    println!("{:?}",result)
}