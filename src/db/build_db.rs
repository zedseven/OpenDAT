use sqlite::Connection;
use std::sync::MutexGuard;

pub fn init(conn: &MutexGuard<Connection>) {
	// Reference Tables
	conn.execute(
		"CREATE TABLE 'system_types' ( \
				'type_id' INT NOT NULL, \
				'name' VARCHAR NOT NULL, \
				PRIMARY KEY ('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'software_categories' ( \
				'category_id' INT NOT NULL, \
				'name' VARCHAR NOT NULL, \
				PRIMARY KEY ('category_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'origin_types' ( \
				'type_id' INT NOT NULL, \
				'name' VARCHAR NOT NULL, \
				PRIMARY KEY ('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'regions' ( \
				'region_id' INT NOT NULL, \
				'region_name' VARCHAR NOT NULL, \
				PRIMARY KEY ('region_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'languages' ( \
				'language_id' INT NOT NULL, \
				'ietf_tag' VARCHAR NOT NULL, \
				PRIMARY KEY ('language_id') \
			);",
	)
	.unwrap();

	// DB Tables
	conn.execute(
		"CREATE TABLE 'systems' ( \
				'system_id' INT NOT NULL, \
				'name' VARCHAR NOT NULL, \
				'name_safe' VARCHAR NULL, \
				'type_id' INT NOT NULL, \
				'wikidata_item_id' VARCHAR NULL, \
				PRIMARY KEY ('system_id') \
				FOREIGN KEY ('type_id') REFERENCES system_types('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'software' ( \
				'software_id' INT NOT NULL, \
				'system_id' INT NOT NULL, \
				'name' VARCHAR NOT NULL, \
				'name_safe' VARCHAR NULL, \
				'category_id' INT NOT NULL, \
				PRIMARY KEY ('software_id'), \
				FOREIGN KEY ('system_id') REFERENCES systems('system_id') \
				FOREIGN KEY ('category_id') REFERENCES software_categories('category_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'software_regions' ( \
				'software_id' INT NOT NULL, \
				'region_id' INT NOT NULL, \
				FOREIGN KEY ('software_id') REFERENCES software('software_id') \
				FOREIGN KEY ('region_id') REFERENCES regions('region_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'versions' ( \
				'version_id' INT NOT NULL, \
				'software_id' INT NOT NULL, \
				'version_string' VARCHAR NULL, \
				'contents' VARCHAR NULL, \
				PRIMARY KEY ('version_id'), \
				FOREIGN KEY ('software_id') REFERENCES software('software_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'version_languages' ( \
				'version_id' INT NOT NULL, \
				'language_id' INT NOT NULL, \
				FOREIGN KEY ('version_id') REFERENCES versions('version_id') \
				FOREIGN KEY ('language_id') REFERENCES languages('language_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'origins' ( \
				'origin_id' INT NOT NULL, \
				'version_id' INT NOT NULL, \
				'sub_origin_id' INT NOT NULL, \
				'type_id' INT NULL, \
				'edition' VARCHAR NULL, \
				PRIMARY KEY ('origin_id'), \
				FOREIGN KEY ('version_id') REFERENCES versions('version_id') \
				FOREIGN KEY ('sub_origin_id') REFERENCES origins('origin_id') \
				FOREIGN KEY ('type_id') REFERENCES origin_types('type_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'dumps' ( \
				'dump_id' INT NOT NULL, \
				'origin_id' INT NOT NULL, \
				'dumper' VARCHAR NULL, \
				'dumper_affiliation' VARCHAR NULL, \
				'dumper_trustworthiness' VARCHAR NULL, \
				'dump_tool' VARCHAR NULL, \
				'dump_tool_version' VARCHAR NULL, \
				'error_count' INT NULL, \
				PRIMARY KEY ('dump_id'), \
				FOREIGN KEY ('origin_id') REFERENCES origins('origin_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'files' ( \
				'file_id' INT NOT NULL, \
				'file_name' VARCHAR NOT NULL, \
				'type_general' VARCHAR NULL, \
				'type_specific' VARCHAR NULL, \
				'downloadable' BOOLEAN NULL, \
				'size' INT NOT NULL, \
				'crc32' BLOB NOT NULL, \
				'md5' BLOB NOT NULL, \
				'sha1' BLOB NOT NULL, \
				'sha256' BLOB NOT NULL, \
				'serial' VARCHAR NULL, \
				'bad' BOOLEAN NULL, \
				'added_date' DATETIME NOT NULL, \
				'file_last_modified' DATETIME NULL, \
				PRIMARY KEY ('file_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'dump_files' ( \
				'dump_id' INT NOT NULL, \
				'file_id' INT NOT NULL, \
				FOREIGN KEY ('dump_id') REFERENCES dumps('dump_id') \
				FOREIGN KEY ('file_id') REFERENCES files('file_id') \
			);",
	)
	.unwrap();
	conn.execute(
		"CREATE TABLE 'file_sub_files' ( \
				'file_id' INT NOT NULL, \
				'sub_file_id' INT NOT NULL, \
				FOREIGN KEY ('file_id') REFERENCES files('file_id') \
				FOREIGN KEY ('sub_file_id') REFERENCES files('file_id') \
			);",
	)
	.unwrap();
}
