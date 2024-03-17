
use std::time::SystemTime;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::sql_types::SqlType;

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

use crate::schema::products;
use crate::schema::users;


#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User{
    pub id: i32,
    pub name: String,
    pub active: bool,
    pub pass: String,
    pub phone: String,
    pub email: String,
    pub username: String,
    pub lastname: String

}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser<'a>{
    pub name: Option<&'a str>,
    pub active: Option<&'a bool>,
    pub pass: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub username: Option<&'a str>,
    pub lastname: Option<&'a str>,
}

 #[derive(Debug, Queryable, Selectable, SqlType)]
 #[diesel(table_name = crate::schema::products)]
 #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product{
    pub id_product: i32,
    pub name: String,
    pub bar_code: String,
    pub cost_price: BigDecimal,
    pub sales_price: BigDecimal,
    pub promotion_price: Option<BigDecimal>,
    stock: bool,
    created_at: Option<SystemTime>,
    updated_at: Option<SystemTime>
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub struct NewProduct<'a>{
    pub name: &'a str,
    pub bar_code: &'a str,
    pub cost_price: &'a BigDecimal,
    pub sales_price: &'a BigDecimal,
    pub promotion_price: &'a BigDecimal,
    pub stock: bool,
    pub updated_at: &'a SystemTime,
}