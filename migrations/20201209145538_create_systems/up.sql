CREATE TABLE 'systems' (
	'system_id' INTEGER PRIMARY KEY NOT NULL,
	'name' VARCHAR NOT NULL,
	'name_safe' VARCHAR NULL,
	'type_id' INTEGER NOT NULL,
	'wikidata_item_id' VARCHAR NULL,
	FOREIGN KEY ('type_id') REFERENCES system_types('type_id')
);
