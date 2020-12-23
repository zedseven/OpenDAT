mod users;

use super::db::{self, DbConn};
use crate::secu::Session;
use rocket::{fairing::AdHoc, Rocket, Route};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde::Serialize;
use std::time::Duration;

pub fn launchpad() -> Rocket {
	rocket::ignite()
		.attach(DbConn::fairing())
		.attach(
			Session::fairing()
				.with_cookie_name("session")
				.with_lifetime(Duration::new(259200, 0)),
		)
		.attach(Template::custom(|engines| {
			engines.tera.autoescape_on(vec!["tera"]);
		}))
		.attach(AdHoc::on_attach("Database Setup", db::init))
		.mount("/", routes![index])
		.mount(users::Users::PATH, users::Users::ROUTES())
		.mount("/css", StaticFiles::from("static/css"))
		.mount("/js", StaticFiles::from("static/js"))
		.mount("/img", StaticFiles::from("static/img"))
}

#[derive(Serialize)]
struct EmptyContext {}

#[get("/")]
fn index() -> Template {
	Template::render("index", &EmptyContext {})
}

pub trait Routable {
	const PATH: &'static str;
	const ROUTES: &'static dyn Fn() -> Vec<Route>;
}
