use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

const DATABASE_URL: &str = "db/chirper.sqlite";

pub struct State {
    pub db_pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl State {
    pub fn new() -> Self {
        let manager = ConnectionManager::new(DATABASE_URL);
        let db_pool = Pool::builder().build(manager).unwrap();

        Self { db_pool }
    }
}
