CREATE TABLE IF NOT EXISTS Picture (
    FilePath TEXT NOT NULL PRIMARY KEY,
    Label TEXT NOT NULL,
    FileSize INTEGER,
    ModifiedTime INTEGER,
    Rank INTEGER,
    Palette BLOB,
    Selected BOOLEAN,
    Deleted BOOLEAN,
    Cover BOOLEAN);

CREATE TABLE IF NOT EXISTS Tag (
    FilePath TEXT NOT NULL,
    Label TEXT NOT NULL,
    PRIMARY KEY (FilePath, Label));

