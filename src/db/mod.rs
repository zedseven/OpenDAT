pub mod models;
pub mod schema;

use rocket::Rocket;
use rocket_contrib::databases::diesel;

#[database("sqlite_db")]
pub struct DbConn(pub(crate) diesel::SqliteConnection);

diesel_migrations::embed_migrations!("migrations");

pub fn init(rocket: Rocket) -> Result<Rocket, Rocket> {
	let conn: DbConn = DbConn::get_one(&rocket)
		.expect("Should be able to get the database connection during basic initialization.");

	embedded_migrations::run_with_output(&conn.0, &mut std::io::stdout())
		.expect("Something went wrong while running embedded database migrations.");

	Ok(rocket)
}
