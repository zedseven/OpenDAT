use serde::Deserialize;
use sqlite::Connection;
use std::fs;
use std::sync::MutexGuard;
use toml;

#[derive(Deserialize)]
struct ReferenceTableConfig {
	data: Vec<String>,
}

pub fn init(conn: &MutexGuard<Connection>) {
	init_reference_tables(conn);
	init_data_tables(conn);
}

fn add_reference_values(conn: &MutexGuard<Connection>, table_name: &str, column_name: &str) {
	const REFERENCE_FILE_PATH: &str = "reference_values";

	let path = format!("{}/{}{}", REFERENCE_FILE_PATH, table_name, ".toml");
	let config = load_reference_file(path);

	let mut stmt = conn
		.prepare(format!(
			"INSERT INTO '{}' ('{}') \
		VALUES (?)",
			table_name, column_name
		))
		.unwrap();
	for value in config.data {
		stmt.bind(1, &*value).unwrap();
		stmt.next().unwrap();
		stmt.reset().unwrap();
	}
}

fn load_reference_file(path: String) -> ReferenceTableConfig {
	let contents = fs::read_to_string(path).unwrap();
	toml::from_str(&*contents).unwrap()
}

fn init_reference_tables(conn: &MutexGuard<Connection>) {
	conn.execute(
		"CREATE TABLE 'system_types' ( \
				'type_id' INTEGER PRIMARY KEY NOT NULL, \
				'name' VARCHAR NOT NULL \
			);",
	)
	.unwrap();
	add_reference_values(conn, "system_types", "name");
	conn.execute(
		"CREATE TABLE 'software_categories' ( \
				'category_id' INTEGER PRIMARY KEY NOT NULL, \
				'name' VARCHAR NOT NULL \
			);",
	)
	.unwrap();
	add_reference_values(conn, "software_categories", "name");
	conn.execute(
		"CREATE TABLE 'origin_types' ( \
				'type_id' INTEGER PRIMARY KEY NOT NULL, \
				'name' VARCHAR NOT NULL \
			);",
	)
	.unwrap();
	add_reference_values(conn, "origin_types", "name");
	conn.execute(
		"CREATE TABLE 'regions' ( \
				'region_id' INTEGER PRIMARY KEY NOT NULL, \
				'region_name' VARCHAR NOT NULL \
			);",
	)
	.unwrap();
	add_reference_values(conn, "regions", "region_name");
	conn.execute(
		"CREATE TABLE 'languages' ( \
				'language_id' INTEGER PRIMARY KEY NOT NULL, \
				'ietf_tag' VARCHAR NOT NULL \
			);",
	)
	.unwrap();
	add_reference_values(conn, "languages", "ietf_tag");
}

fn init_data_tables(conn: &MutexGuard<Connection>) {
	conn.execute(
		"CREATE TABLE 'systems' ( \
				'system_id' INTEGER PRIMARY KEY NOT NULL, \
				'name' VARCHAR NOT NULL, \
				'name_safe' VARCHAR NULL, \
				'type_id' INTEGER NOT NULL, \
				'wikidata_item_id' VARCHAR NULL, \
				FOREIGN KEY ('type_id') REFERENCES system_types('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'software' ( \
				'software_id' INTEGER PRIMARY KEY NOT NULL, \
				'system_id' INTEGER NOT NULL, \
				'name' VARCHAR NOT NULL, \
				'name_safe' VARCHAR NULL, \
				'category_id' INTEGER NOT NULL, \
				FOREIGN KEY ('system_id') REFERENCES systems('system_id'), \
				FOREIGN KEY ('category_id') REFERENCES software_categories('category_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'software_regions' ( \
				'software_id' INTEGER NOT NULL, \
				'region_id' INTEGER NOT NULL, \
				PRIMARY KEY ('software_id', 'region_id') \
				FOREIGN KEY ('software_id') REFERENCES software('software_id'), \
				FOREIGN KEY ('region_id') REFERENCES regions('region_id') \
			)  WITHOUT ROWID;",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'versions' ( \
				'version_id' INTEGER PRIMARY KEY NOT NULL, \
				'software_id' INTEGER NOT NULL, \
				'version_string' VARCHAR NULL, \
				'contents' VARCHAR NULL, \
				FOREIGN KEY ('software_id') REFERENCES software('software_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'version_languages' ( \
				'version_id' INTEGER NOT NULL, \
				'language_id' INTEGER NOT NULL, \
				PRIMARY KEY ('version_id', 'language_id') \
				FOREIGN KEY ('version_id') REFERENCES versions('version_id'), \
				FOREIGN KEY ('language_id') REFERENCES languages('language_id') \
			) WITHOUT ROWID;",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'origins' ( \
				'origin_id' INTEGER PRIMARY KEY NOT NULL, \
				'version_id' INTEGER NOT NULL, \
				'sub_origin_id' INTEGER NOT NULL, \
				'type_id' INTEGER NULL, \
				'edition' VARCHAR NULL, \
				FOREIGN KEY ('version_id') REFERENCES versions('version_id'), \
				FOREIGN KEY ('sub_origin_id') REFERENCES origins('origin_id'), \
				FOREIGN KEY ('type_id') REFERENCES origin_types('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'dumps' ( \
				'dump_id' INTEGER PRIMARY KEY NOT NULL, \
				'origin_id' INTEGER NOT NULL, \
				'dumper' VARCHAR NULL, \
				'dumper_affiliation' VARCHAR NULL, \
				'dumper_trustworthiness' VARCHAR NULL, \
				'dump_tool' VARCHAR NULL, \
				'dump_tool_version' VARCHAR NULL, \
				'error_count' INTEGER NULL, \
				FOREIGN KEY ('origin_id') REFERENCES origins('origin_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'files' ( \
				'file_id' INTEGER PRIMARY KEY NOT NULL, \
				'file_name' VARCHAR NOT NULL, \
				'type_general' VARCHAR NULL, \
				'type_specific' VARCHAR NULL, \
				'downloadable' BOOLEAN NULL, \
				'size' INTEGER NOT NULL, \
				'crc32' BLOB NOT NULL, \
				'md5' BLOB NOT NULL, \
				'sha1' BLOB NOT NULL, \
				'sha256' BLOB NOT NULL, \
				'serial' VARCHAR NULL, \
				'bad' BOOLEAN NULL, \
				'added_date' DATETIME NOT NULL, \
				'file_last_modified' DATETIME NULL \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'dump_files' ( \
				'dump_id' INTEGER NOT NULL, \
				'file_id' INTEGER NOT NULL, \
				PRIMARY KEY ('dump_id', 'file_id') \
				FOREIGN KEY ('dump_id') REFERENCES dumps('dump_id'), \
				FOREIGN KEY ('file_id') REFERENCES files('file_id') \
			) WITHOUT ROWID;",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'file_sub_files' ( \
				'file_id' INTEGER NOT NULL, \
				'sub_file_id' INTEGER NOT NULL, \
				PRIMARY KEY ('file_id', 'sub_file_id') \
				FOREIGN KEY ('file_id') REFERENCES files('file_id'), \
				FOREIGN KEY ('sub_file_id') REFERENCES files('file_id') \
			) WITHOUT ROWID;",
	)
	.unwrap();
}
