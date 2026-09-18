UPDATE Folder
SET FirstFilePath = (
    SELECT Picture.FilePath
    FROM Picture
    WHERE Picture.FolderId = Folder.FolderId
      AND Picture.Cover > 0
)
WHERE EXISTS (
    SELECT 1
    FROM Picture
    WHERE Picture.FolderId = Folder.FolderId
      AND Picture.Cover > 0
);
