use surrealdb::Surreal;
use surrealdb::engine::local::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Db>,
}
