use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct ServerState {
    pub db_conn: DatabaseConnection,
}
