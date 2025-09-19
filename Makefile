create_db: sql/create_db.sql
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db ".schema"
