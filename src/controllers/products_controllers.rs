use crate::models::UpdateProduct;
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
    let result = diesel::update(products.filter(id_product.eq(id_finder)))
        .set(&update_product)
        .execute(conn)
        .expect("Error al hacer un update al producto");
    println!("Update completado {:?}", result);
}

pub fn db_get_product_by_id(connection: &mut PgConnection, product_id: i32)-> Option<Product>{
    use crate::schema::products::dsl::*;
    let product = products
        .filter(id_product.eq(product_id))
        .first::<Product>(connection)
        .ok();
    product
}

pub fn db_delete_product(conn: &mut PgConnection, bar_code_finder:&str){
    use crate::schema::products::dsl::*;
    diesel::delete(products.filter(bar_code.eq(bar_code_finder)))
    .execute(conn)
    .expect("Error al eliminar el producto de la base de datos.");
    println!("db_delete_product Ejecutado");    
}

pub fn db_update_quantity_product(conn: &mut PgConnection, bar_code_finder:&str, update_quantity_product: UpdateProduct){
    use crate::schema::products::dsl::*;
    diesel::update(products.filter(bar_code.eq(bar_code_finder)))
        .set(&update_quantity_product)
        .execute(conn)
        .expect("Error al agregar producto al inventario");

}