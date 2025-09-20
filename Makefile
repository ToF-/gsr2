create_db: sql/create_db.sql
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db ".schema"
	sqlite3 testdata/gsr2.db <sql/insert_test_data.sql
	sqlite3 testdata/gsr2.db "SELECT * FROM Picture ORDER BY FilePath;"
