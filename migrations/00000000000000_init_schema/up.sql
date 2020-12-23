-- Reference Tables --

CREATE TABLE 'system_types' (
	'type_id' INTEGER PRIMARY KEY NOT NULL,
	'type_name' VARCHAR NOT NULL
);

CREATE TABLE 'software_categories' (
	'category_id' INTEGER PRIMARY KEY NOT NULL,
	'category_name' VARCHAR NOT NULL
);

CREATE TABLE 'origin_types' (
	'type_id' INTEGER PRIMARY KEY NOT NULL,
	'type_name' VARCHAR NOT NULL
);

CREATE TABLE 'regions' (
	'region_id' INTEGER PRIMARY KEY NOT NULL,
	'region_name' VARCHAR NOT NULL
);

CREATE TABLE 'languages' (
	'language_id' INTEGER PRIMARY KEY NOT NULL,
	'ietf_tag' VARCHAR NOT NULL
);


-- Data Tables --

CREATE TABLE 'users' (
	'user_name' VARCHAR PRIMARY KEY NOT NULL,
	'user_pass_hash' VARCHAR NOT NULL,
	'disabled' BOOLEAN NOT NULL DEFAULT 0,
	'email' VARCHAR
);

CREATE TABLE 'systems' (
	'system_id' INTEGER PRIMARY KEY NOT NULL,
	'name' VARCHAR NOT NULL,
	'name_safe' VARCHAR NULL,
	'type_id' INTEGER NOT NULL,
	'wikidata_item_id' VARCHAR NULL,
	FOREIGN KEY ('type_id') REFERENCES system_types('type_id')
);

CREATE TABLE 'software' (
	'software_id' INTEGER PRIMARY KEY NOT NULL,
	'system_id' INTEGER NOT NULL,
	'name' VARCHAR NOT NULL,
	'name_safe' VARCHAR NULL,
	'category_id' INTEGER NOT NULL,
	FOREIGN KEY ('system_id') REFERENCES systems('system_id'),
	FOREIGN KEY ('category_id') REFERENCES software_categories('category_id')
);

CREATE TABLE 'software_regions' (
	'software_id' INTEGER NOT NULL,
	'region_id' INTEGER NOT NULL,
	PRIMARY KEY ('software_id', 'region_id'),
	FOREIGN KEY ('software_id') REFERENCES software('software_id'),
	FOREIGN KEY ('region_id') REFERENCES regions('region_id')
)  WITHOUT ROWID;

CREATE TABLE 'versions' (
	'version_id' INTEGER PRIMARY KEY NOT NULL,
	'software_id' INTEGER NOT NULL,
	'version_string' VARCHAR NULL,
	'contents' VARCHAR NULL,
	FOREIGN KEY ('software_id') REFERENCES software('software_id')
);

CREATE TABLE 'version_languages' (
	'version_id' INTEGER NOT NULL,
	'language_id' INTEGER NOT NULL,
	PRIMARY KEY ('version_id', 'language_id'),
	FOREIGN KEY ('version_id') REFERENCES versions('version_id'),
	FOREIGN KEY ('language_id') REFERENCES languages('language_id')
) WITHOUT ROWID;

CREATE TABLE 'origins' (
	'origin_id' INTEGER PRIMARY KEY NOT NULL,
	'version_id' INTEGER NOT NULL,
	'parent_origin_id' INTEGER NULL,
	'type_id' INTEGER NULL,
	'edition' VARCHAR NULL,
	FOREIGN KEY ('version_id') REFERENCES versions('version_id'),
	FOREIGN KEY ('parent_origin_id') REFERENCES origins('origin_id'),
	FOREIGN KEY ('type_id') REFERENCES origin_types('type_id')
);

CREATE TABLE 'dumps' (
	'dump_id' INTEGER PRIMARY KEY NOT NULL,
	'origin_id' INTEGER NOT NULL,
	'dumper' VARCHAR NULL,
	'dumper_affiliation' VARCHAR NULL,
	'dumper_trustworthiness' VARCHAR NULL,
	'dump_tool' VARCHAR NULL,
	'dump_tool_version' VARCHAR NULL,
	'error_count' INTEGER NULL,
	FOREIGN KEY ('origin_id') REFERENCES origins('origin_id')
);

CREATE TABLE 'files' (
	'file_id' INTEGER PRIMARY KEY NOT NULL,
	'file_name' VARCHAR NOT NULL,
	'type_general' VARCHAR NULL,
	'type_specific' VARCHAR NULL,
	'downloadable' BOOLEAN NULL,
	'size' INTEGER NOT NULL,
	'crc32' BLOB NOT NULL,
	'md5' BLOB NOT NULL,
	'sha1' BLOB NOT NULL,
	'sha256' BLOB NOT NULL,
	'serial' VARCHAR NULL,
	'bad' BOOLEAN NULL,
	'added_date' DATETIME NOT NULL,
	'file_last_modified' DATETIME NULL
);

CREATE TABLE 'dump_files' (
	'dump_id' INTEGER NOT NULL,
	'file_id' INTEGER NOT NULL,
	PRIMARY KEY ('dump_id', 'file_id'),
	FOREIGN KEY ('dump_id') REFERENCES dumps('dump_id'),
	FOREIGN KEY ('file_id') REFERENCES files('file_id')
) WITHOUT ROWID;

CREATE TABLE 'file_sub_files' (
	'file_id' INTEGER NOT NULL,
	'sub_file_id' INTEGER NOT NULL,
	PRIMARY KEY ('file_id', 'sub_file_id'),
	FOREIGN KEY ('file_id') REFERENCES files('file_id'),
	FOREIGN KEY ('sub_file_id') REFERENCES files('file_id')
) WITHOUT ROWID;
