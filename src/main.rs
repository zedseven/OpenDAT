#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db;
mod routes;
mod secu;

fn main() {
	routes::launchpad().launch();
}
