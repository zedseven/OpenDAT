CREATE TABLE 'file_sub_files' (
	'file_id' INTEGER NOT NULL,
	'sub_file_id' INTEGER NOT NULL,
	PRIMARY KEY ('file_id', 'sub_file_id')
	FOREIGN KEY ('file_id') REFERENCES files('file_id'),
	FOREIGN KEY ('sub_file_id') REFERENCES files('file_id')
) WITHOUT ROWID;
