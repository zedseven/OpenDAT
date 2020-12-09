CREATE TABLE 'versions' (
	'version_id' INTEGER PRIMARY KEY NOT NULL,
	'software_id' INTEGER NOT NULL,
	'version_string' VARCHAR NULL,
	'contents' VARCHAR NULL,
	FOREIGN KEY ('software_id') REFERENCES software('software_id')
);
