pub async fn list_conversions_handler(db_pool: DBPool) -> Result<impl Reply> {
    let conversions = db::conversion::fetch(&db_pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &conversions.into_iter().map(conversionResponse::of).collect(),
    ))
}

pub async fn fetch_conversion_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let conversion = db::conversion::fetch_one(&db_pool, id)
        .await
        .map_err(reject::custom)?;
    Ok(json(&conversionResponse::of(conversion)))
}

pub async fn create_conversion_handler(body: conversionRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&conversionResponse::of(
        db::conversion::create(&db_pool, body)
            .await
            .map_err(reject::custom)?,
    )))
}
