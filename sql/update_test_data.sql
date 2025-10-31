UPDATE Picture
    SET Label = 'nine-colors', Rank = 0, Cover = true 
WHERE FilePath = '~/Coding/gsr2/testdata/nine_colors.png';

UPDATE Picture
    SET Label = 'white_square', Rank = 2, Cover = false 
WHERE FilePath = '~/Coding/gsr2/testdata/white_square.png';

UPDATE Picture
    SET Label = 'large_picture', Rank = 4, Cover = false
WHERE FilePath = '~/Coding/gsr2/testdata/large_picture.png';

UPDATE Picture
    SET Label = 'dot', Rank = 4, Cover = false
WHERE FilePath = '~/Coding/gsr2/testdata/single_dot.png';

DELETE FROM Tag;

INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/nine_colors.png', 'foo');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/nine_colors.png', 'bar');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/nine_colors.png', 'qux');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/white_square.png', 'foo');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/white_square.png', 'bar');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/single_dot.png', 'bar');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/single_dot.png', 'a_rather_long_tag');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/large_picture.png', 'a');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/large_picture.png', 'bunch');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/large_picture.png', 'of');
INSERT INTO Tag (FilePath, Label) VALUES ('~/Coding/gsr2/testdata/large_picture.png', 'tags');
