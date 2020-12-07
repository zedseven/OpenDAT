#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use sqlite;
use std::sync::Mutex;

lazy_static! {
	static ref CONN: Mutex<sqlite::Connection> =
		{ Mutex::new(sqlite::open("./db.sqlite").unwrap()) };
}

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	CONN.lock().unwrap();
	rocket::ignite().mount("/", routes![index]).launch();
}
