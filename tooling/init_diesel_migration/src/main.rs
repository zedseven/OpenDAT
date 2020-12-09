use chrono::prelude::*;
use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		println!("args: location_path migration_name");
		return;
	}
	let location_path = &args[1];
	let migration_name = &args[2];

	let local_date_time = Local::now();
	let dir_path = &*format!(
		"{}/{}_{}",
		location_path,
		local_date_time.format("%Y%m%d%H%M%S"),
		migration_name
	);
	println!("Dir Path: {}", dir_path);
	fs::create_dir(dir_path).expect("Couldn't create the directory.");
	fs::write(format!("{}/up.sql", dir_path), "\n").expect("Couldn't create up.sql.");
	fs::write(format!("{}/down.sql", dir_path), "\n").expect("Couldn't create down.sql.");
}
