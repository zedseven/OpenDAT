#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

mod db;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	db::init_if_necessary();
	rocket::ignite().mount("/", routes![index]).launch();
}
