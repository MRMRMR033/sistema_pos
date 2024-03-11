use crate::models::{NewUser, User};
use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, name: &str, lastname: &str, email: &str, cel: &str, house_cel: &str, active: bool, admin:bool )-> User{
    use crate::schema::users;
    let new_user = NewUser {name, lastname, email, cel, house_cel, active: &active, admin: &admin};

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error al aguardar el usuario")
}

pub fn update_user(connection: PgConnection, id_user:i32, new_email: Option<String>, new_cel: Option<&str>, new_house_cel: Option<&str>, new_active: Option<bool>, new_admin: Option<bool>){
    use crate::schema::users::dsl::*;
    let mut query = diesel::update(users.filter(id.eq(id_user)));


    if let Some(email) = new_email{
        query = query.set(email.eq(email));
    }

    if let Some(cel) = new_cel {
        query = query.set(cel.eq(cel));
    }

    if let Some(house_cel) = new_house_cel {
        query = query.set(house_cel.eq(house_cel));
    }

    if let Some(active) = new_active {
        query = query.set(active.eq(active));
    }

    if let Some(admin) = new_admin {
        query = query
    }

    query.execute(connection).expect("Error al actualizar el usuario");
}