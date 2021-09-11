use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool};
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "conversions";
const SELECT_FIELDS: &str = "id, name, formula";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<Conversion>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_conversion(&r)).collect())
}


pub async fn fetch_one(db_pool: &DBPool, id: i32) -> Result<Conversion> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS, TABLE);

    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_conversion(&row))
}

pub async fn create(db_pool: &DBPool, body: ConversionRequest) -> Result<Conversion> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_conversion(&row))
}

pub async fn delete(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_conversion(row: &Row) -> Conversion {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let formula: String = row.get(2);
    Conversion { id, name, formula }
}
