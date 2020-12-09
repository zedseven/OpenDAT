use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
pub struct DbConn(pub(crate) diesel::SqliteConnection);
