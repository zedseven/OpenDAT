mod build_db;

use sqlite;
use std::sync::Mutex;

const CHECK_TABLE: &str = "systems"; // A table that will always exist in the DB.

lazy_static! {
	pub static ref CONN: Mutex<sqlite::Connection> = Mutex::new(sqlite::open("db.sqlite").unwrap());
}

pub fn init_if_necessary() {
	let conn = CONN.lock().unwrap();
	match conn.prepare(format!(
		"SELECT \
				name \
			FROM '{}'",
		CHECK_TABLE
	)) {
		Ok(_) => {
			println!("DB already exists. No initialization necessary.");
			return;
		}
		_ => {
			println!("DB doesn't exist yet - initializing now.");
			build_db::init(&conn);
		}
	};
}
