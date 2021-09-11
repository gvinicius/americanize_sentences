use crate::{db, DBPool, Result};
use common::*;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn list_conversions_handler(db_pool: DBPool) -> Result<impl Reply> {
    let conversions = db::conversion::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &conversions.into_iter().map(ConversionResponse::of).collect(),
    ))
}

pub async fn fetch_conversion_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let conversion = db::conversion::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&ConversionResponse::of(conversion)))
}

pub async fn create_conversion_handler(body: ConversionRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&ConversionResponse::of(
        db::conversion::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}
