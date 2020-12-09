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
