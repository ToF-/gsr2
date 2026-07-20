ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

reinit_sql:
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db <sql/update_test_data.sql
	sqlite3 testdata/gsr2.db "SELECT RowId, FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	sqlite3 testdata/gsr2.db "SELECT RowId, FilePath, Label FROM Tag;"

show_sql:
	script/show_sql

reinit_data:
	rm -rf testdata
	mkdir -p testdata
	mkdir -p testdata/subdir
	echo '(- (foo bar qux) (bog gap) (pat (jxs lam lom lum) (zzz tic tac toe) (pin blo) ))'>testdata/catalog.sexp
	sqlite3 testdata/gsr2.db <sql/create_db.sql
	sqlite3 testdata/gsr2.db <sql/update_test_data.sql
	sqlite3 testdata/gsr2.db ".schema"
	cargo run --bin gen_data
	cargo run -- collect testdata
	cp test_thumbs/* testdata/.
	sqlite3 testdata/gsr2.db <sql/update_test_data.sql
	sqlite3 testdata/gsr2.db "SELECT RowId, FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	sqlite3 testdata/gsr2.db "SELECT RowId, FilePath, Label FROM Tag;"
	tree testdata
	
install:
	cargo install --path .

updates:
	sqlite3 testdata/gsr2.db <sql/update_test_data.sql

