use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub cel: String,
    pub house_cel: String,
    pub active: bool,
    pub admin: bool
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
    pub name: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub cel: &'a str,
    pub house_cel: &'a str,
    pub active: &'a bool,
    pub admin: &'a bool,
}