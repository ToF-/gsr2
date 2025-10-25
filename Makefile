create_db: sql/create_db.sql
	rm testdata/gsr2.db
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db ".schema"

gen_data: src/gen_data.rs
	cargo run --bin gen_data
	cargo run -- thumbnails 10
	cargo run -- thumbnails 7
	cargo run -- thumbnails 4
	cargo run -- thumbnails 2

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
collect_data:
	cargo run -- collect $(ROOT_DIR)/testdata

reinit_data: create_db gen_data collect_data
	sqlite3 testdata/gsr2.db "SELECT FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	
