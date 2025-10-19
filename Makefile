create_db: sql/create_db.sql
	rm testdata/gsr2.db
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db ".schema"
	sqlite3 testdata/gsr2.db <sql/insert_test_data.sql
	sqlite3 testdata/gsr2.db "SELECT * FROM Picture ORDER BY FilePath;"

gen_data: src/gen_data.rs
	cargo run --bin gen_data
	cargo run -- --create 10
	cargo run -- --create 7
	cargo run -- --create 4
	cargo run -- --create 2

reinit_data: create_db gen_data
	
