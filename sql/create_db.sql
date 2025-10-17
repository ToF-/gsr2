CREATE TABLE IF NOT EXISTS Picture (
    FilePath TEXT NOT NULL PRIMARY KEY,
    Label TEXT NOT NULL,
    FileSize INTEGER,
    ModifiedTime INTEGER,
    Rank INTEGER,
    SampleSize INTEGER,
    Sample BLOB,
    ColorCount INTEGER,
    Cover BOOLEAN);

CREATE TABLE IF NOT EXISTS Tag (
    FilePath TEXT NOT NULL,
    Label TEXT NOT NULL,
    PRIMARY KEY (FilePath, Label));

