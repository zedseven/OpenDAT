use super::Routable;
use crate::db::{self, models, DbConn};
use crate::secu::{self, Session};
use diesel::prelude::*;
use rocket::{
	request::{FlashMessage, Form},
	response::{Flash, Redirect},
	Route,
};
use rocket_contrib::templates::Template;
use serde::Serialize;

// Structs
#[derive(Serialize)]
struct DisplayUser {
	name: String,
	email: Option<String>,
	disabled: bool,
}

#[derive(Serialize)]
struct UsersContext {
	user_list: Vec<DisplayUser>,
}

#[derive(FromForm)]
struct UserPass {
	user_name: String,
	password: String,
}

// Routes
#[get("/")]
fn get_users(conn: DbConn) -> Template {
	use db::schema::users::dsl::*;

	let mut context = UsersContext { user_list: vec![] };
	let results = users
		.load::<models::User>(&conn.0)
		.expect("Unable to load users.");
	for user in results {
		context.user_list.push(DisplayUser {
			name: user.user_name,
			email: user.email,
			disabled: user.disabled,
		})
	}
	Template::render("users", &context)
}

#[post("/", data = "<user_input>")]
fn add_user(conn: DbConn, user_input: Form<UserPass>) -> Redirect {
	use db::schema::users;

	let new_user = models::User {
		user_name: user_input.user_name.clone(),
		user_pass_hash: secu::hash(&*user_input.password),
		disabled: false,
		email: None,
	};

	let insert_result = diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&conn.0);

	// TODO: Redirect somewhere else on error
	match insert_result {
		Ok(_) => Redirect::to("/users"),
		_ => Redirect::to("/users"),
	}
}

#[derive(Serialize)]
struct LoginContext {
	flash_message: String,
}

#[get("/login")]
fn get_login(session: Session, flash: Option<FlashMessage>) -> Result<Template, Redirect> {
	return if session.tap(|session_user| *session_user == None) {
		Ok(Template::render(
			"login",
			&LoginContext {
				flash_message: flash.map(|msg| String::from(msg.msg())).unwrap_or_default(),
			},
		))
	} else {
		Err(Redirect::to("/users"))
	};
}

#[post("/login", data = "<user_input>")]
fn log_in(
	session: Session,
	conn: DbConn,
	user_input: Form<UserPass>,
) -> Result<Redirect, Flash<Redirect>> {
	use db::schema::users::dsl::*;

	if session.tap(|session_user| *session_user != None) {
		return Err(Flash::error(Redirect::to("/users"), "Already logged in."));
	}

	let pass_hash: String;
	let user_result = users
		.select(user_pass_hash)
		.filter(user_name.eq(&user_input.user_name))
		.first(&conn.0);
	match user_result {
		Ok(hash) => pass_hash = hash,
		Err(_) => {
			return Err(Flash::error(
				Redirect::to("/users/login"),
				"Username or password is incorrect.",
			))
		}
	}

	if secu::verify(&*pass_hash, &*user_input.password) {
		session.tap(|session_user| *session_user = Some(user_input.user_name.clone()));
		Ok(Redirect::to("/users"))
	} else {
		Err(Flash::error(
			Redirect::to("/users/login"),
			"Username or password is incorrect.",
		))
	}
}

pub struct Users {}
impl Routable for Users {
	const PATH: &'static str = "/users";
	const ROUTES: &'static dyn Fn() -> Vec<Route> =
		&|| routes![get_users, add_user, get_login, log_in];
}
