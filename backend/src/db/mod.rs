type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let host = env::var("PGHOST")
    let user = env::var("PGUSER")
    let psw = env::var("PGPASS")
    let dbname = env::var("PGDBNAME")
    let address = user + "://" + psw + "@" + host + "5432" + "/" + dbname
    let config = Config::from_str(address)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
       .max_open(DB_POOL_MAX_OPEN)
       .max_idle(DB_POOL_MAX_IDLE)
       .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
       .build(manager))
}