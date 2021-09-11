mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let pet = warp::path!("conversion" / i32 / "pet");
    let pet_param = warp::path!("conversion" / i32 / "pet" / i32);
    let conversion = warp::path("conversion");

    let pet_routes = pet
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_pets_handler)
        .or(pet
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_pet_handler))
        .or(pet_param
            .and(warp::delete())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_pet_handler));

    let conversion_routes = conversion
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and_then(handler::fetch_conversion_handler)
        .or(conversion
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_conversions_handler))
        .or(conversion
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_conversion_handler));

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
