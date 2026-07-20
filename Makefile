ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
TEST_DIR:=$(ROOT_DIR)"/testdata"
CATALOG:=$(TEST_DIR)"/catalog.sexp"
TEMP_DIR:=$(TEST_DIR)"/subdir"
SQLITE_PARAM:=".param set :test_dir "$(TEST_DIR)
DATABASE:=$(TEST_DIR)"/gsr2.db"
OS_NAME := $(shell uname -s | tr A-Z a-z)

ifeq ($(OS_NAME),darwin)
	SED := "gsed"
else
	SED := sed
endif

info:
	echo $(OS_NAME)
	echo $(ROOT_DIR)
	echo $(TEST_DIR)

reinit_sql:
	sqlite3 $(DATABASE) <sql/create_db.sql
	sqlite3 $(DATABASE) $(SQLITE_PARAM) ".read sql/update_test_data.sql" 
	sqlite3 $(DATABASE) "SELECT RowId, FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	sqlite3 $(DATABASE) "SELECT RowId, FilePath, Label FROM Tag;"

show_sql:
	script/show_sql

reinit_data:
	echo "base_dir = \"$(TEST_DIR)\"" >script/variables.toml
	echo "database_file = \"$(DATABASE)\"" >>script/variables.toml
	echo "catalog_filepath = \"$(CATALOG)\"" >>script/variables.toml
	echo "temp_dir = \"$(TEMP_DIR)"\" >>script/variables.toml
	cat script/variables.toml script/gsr-template.toml >script/gsr2.toml
	rm -rf $(TEST_DIR)
	mkdir -p $(TEST_DIR)
	mkdir -p $(TEST_DIR)/subdir
	echo '(- (foo bar qux) (bog gap) (pat (jxs lam lom lum) (zzz tic tac toe) (pin blo) ))' >$(TEST_DIR)/catalog.sexp
	sqlite3 $(DATABASE) ".read sql/create_db.sql"
	sqlite3 $(DATABASE) ".schema"
	cargo run --bin gen_data
	cargo run -- collect $(TEST_DIR)
	cp test_thumbs/* testdata/.
	sqlite3 $(DATABASE) $(SQLITE_PARAM) ".read sql/update_test_data.sql"
	sqlite3 $(DATABASE) "SELECT RowId, FilePath, Label, FileSize, ModifiedTime, Rank, ColorCount, Cover FROM Picture;"
	sqlite3 $(DATABASE) "SELECT RowId, FilePath, Label FROM Tag;"
	tree $(TEST_DIR)
	
install:
	cargo install --path .

updates:
	sqlite3 $(DATABASE) ".read sql/update_test_data.sql"

