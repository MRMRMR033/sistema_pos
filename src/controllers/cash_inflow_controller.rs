use crate::models::CashInflow;
use crate::schema::cash_inflow;
use crate::models::NewCashInflow;

use diesel;
use diesel::dsl::all;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;

pub fn db_create_cash_in_flow(conn: &mut PgConnection, new_cash_in_flow: NewCashInflow)-> CashInflow{
    diesel::insert_into(cash_inflow::table)
        .values(&new_cash_in_flow)
        .returning(CashInflow::as_returning())
        .get_result(conn)
        .expect("Error al guardar la entrada de efectivo")
}

pub fn db_get_all_cash_in_flow(conn: &mut PgConnection, user_id_finder: i32)-> Vec<CashInflow>{
    use crate::schema::cash_inflow::dsl::*;
    let all_movements: Vec<CashInflow> = cash_inflow.load::<CashInflow>(conn)
    .expect("error al obtener las fechas");
    all_movements
}