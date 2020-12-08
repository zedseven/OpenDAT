#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

mod db;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	db::init_if_necessary();
	rocket::ignite()
		.mount("/", routes![index])
		.mount("/css", StaticFiles::from("static/css"))
		.mount("/js", StaticFiles::from("static/js"))
		.mount("/img", StaticFiles::from("static/img"))
		.launch();
}
