CREATE TABLE 'dump_files' (
	'dump_id' INTEGER NOT NULL,
	'file_id' INTEGER NOT NULL,
	PRIMARY KEY ('dump_id', 'file_id')
	FOREIGN KEY ('dump_id') REFERENCES dumps('dump_id'),
	FOREIGN KEY ('file_id') REFERENCES files('file_id')
) WITHOUT ROWID;
