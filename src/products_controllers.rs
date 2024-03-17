use crate::schema::products;
use crate::models::NewProduct;
use crate::models::Product;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn db_create_product(conn: &mut PgConnection, new_product: NewProduct )-> Product{
    diesel::insert_into(products::table)
        .values(&new_product)
        .returning( Product::as_returning())
        .get_result(conn)
        .expect("Error al aguardar el producto")
}

pub fn db_update_product(conn: &mut PgConnection, id_finder:i32, update_product: NewProduct){
    use crate::schema::products::dsl::*;
}