CREATE TABLE 'version_languages' (
	'version_id' INTEGER NOT NULL,
	'language_id' INTEGER NOT NULL,
	PRIMARY KEY ('version_id', 'language_id')
	FOREIGN KEY ('version_id') REFERENCES versions('version_id'),
	FOREIGN KEY ('language_id') REFERENCES languages('language_id')
) WITHOUT ROWID;
