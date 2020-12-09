CREATE TABLE 'software_regions' (
	'software_id' INTEGER NOT NULL,
	'region_id' INTEGER NOT NULL,
	PRIMARY KEY ('software_id', 'region_id')
	FOREIGN KEY ('software_id') REFERENCES software('software_id'),
	FOREIGN KEY ('region_id') REFERENCES regions('region_id')
)  WITHOUT ROWID;
