use super::db;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

pub fn launchpad() -> Rocket {
	rocket::ignite()
		.attach(db::type_access::DbConn::fairing())
		.attach(Template::custom(|engines| {
			engines.tera.autoescape_on(vec!["html", "sql"]);
		}))
		.attach(AdHoc::on_attach("Database Setup", db::init))
		.mount("/", routes![index])
		.mount("/css", StaticFiles::from("static/css"))
		.mount("/js", StaticFiles::from("static/js"))
		.mount("/img", StaticFiles::from("static/img"))
}
