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
