use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};

pub async fn connect_surrealdb() -> Surreal<Db> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db
}
