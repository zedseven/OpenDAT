use super::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
	pub user_name: String,
	pub user_pass_hash: String,
	pub disabled: bool,
	pub email: Option<String>,
}
