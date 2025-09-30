DELETE FROM Picture;

INSERT INTO Picture (FilePath, Label)
VALUES ('testdata/nine_colors.png', 'sample');

INSERT INTO Picture (FilePath, Label)
VALUES ('testdata/single_dot.png', '');

INSERT INTO Picture (FilePath, Label)
VALUES ('testdata/white_square.png', 'foo');

INSERT INTO Picture (FilePath, Label)
VALUES ('testdata/large_picture.png', 'large-picture');
