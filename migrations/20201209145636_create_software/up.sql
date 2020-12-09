CREATE TABLE 'software' (
	'software_id' INTEGER PRIMARY KEY NOT NULL,
	'system_id' INTEGER NOT NULL,
	'name' VARCHAR NOT NULL,
	'name_safe' VARCHAR NULL,
	'category_id' INTEGER NOT NULL,
	FOREIGN KEY ('system_id') REFERENCES systems('system_id'),
	FOREIGN KEY ('category_id') REFERENCES software_categories('category_id')
);
