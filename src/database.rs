use postgres::{PostgresConnection, NoSsl};
use postgres::error::PostgresConnectError;

static DB_URL: &'static str = "postgresql://festivus@localhost";

pub fn get_db_conn() -> Result<PostgresConnection, PostgresConnectError> {
    PostgresConnection::connect(DB_URL, &NoSsl)
}
