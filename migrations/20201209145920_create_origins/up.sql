CREATE TABLE 'origins' (
	'origin_id' INTEGER PRIMARY KEY NOT NULL,
	'version_id' INTEGER NOT NULL,
	'sub_origin_id' INTEGER NOT NULL,
	'type_id' INTEGER NULL,
	'edition' VARCHAR NULL,
	FOREIGN KEY ('version_id') REFERENCES versions('version_id'),
	FOREIGN KEY ('sub_origin_id') REFERENCES origins('origin_id'),
	FOREIGN KEY ('type_id') REFERENCES origin_types('type_id')
);
