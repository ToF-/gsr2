ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
reinit_data:
	rm -rf testdata
	mkdir -p testdata
	mkdir -p testdata/subdir
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db ".schema"
	cargo run --bin gen_data
	cargo run -- collect $(ROOT_DIR)/testdata
	cargo run -- thumbnails 10
	cargo run -- thumbnails 7
	cargo run -- thumbnails 4
	cargo run -- thumbnails 2
	sqlite3 testdata/gsr2.db "SELECT FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	tree testdata
	
install:
	cargo install --path .
