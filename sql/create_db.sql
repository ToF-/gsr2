CREATE TABLE IF NOT EXISTS Picture (
    FilePath TEXT NOT NULL PRIMARY KEY,
    Label TEXT NOT NULL,
    FileSize INTEGER,
    ModifiedTime INTEGER,
    Rank INTEGER,
    Sample BLOB,
    ColorCount INTEGER,
    Cover BOOLEAN,
    Score INTEGER NOT NULL DEFAULT 0,
    Category TEXT,
    FolderId INTEGER NOT NULL DEFAULT 0);

CREATE TABLE IF NOT EXISTS Tag (
    FilePath TEXT NOT NULL,
    Label TEXT NOT NULL,
    PRIMARY KEY (FilePath, Label));

CREATE TABLE IF NOT EXISTS Folder (
    FolderId INTEGER NOT NULL PRIMARY KEY,
    FilePath TEXT UNIQUE,
    ParentId INTEGER NOT NULL,
    PictureCount INTEGER NOT NULL);

CREATE INDEX IF NOT EXISTS idx_picture_folder ON Picture(FolderId);
