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

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id: i32,
    pub name: String,
    pub active: bool,

}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
    pub name: &'a str,
    pub active: &'a bool,
}