pub const DEFAULT_HEIGHT: i32 = 1000;
pub const DEFAULT_WIDTH: i32 = 1000;
pub const ENTRY_WINDOW_WIDTH: i32 = 250;
pub const ENTRY_WINDOW_HEIGHT: i32 = 100;
pub const ENTRY_CURSOR_1: char = '▪';
pub const ENTRY_CURSOR_2: char = '▫';
pub const FOCUS_SYMBOL_1: char = '⭓'; // '◆';
pub const FOCUS_SYMBOL_2: char = '⭔'; // '◇';
pub const COVER_SYMBOL: &str = "🔶";
pub const ORDER_SYMBOL: &str = "↑";
pub const THREE_STARS_SYMBOL: &str = "☆☆☆";
pub const TWO_STARS_SYMBOL: &str = "☆☆";
pub const ONE_STAR_SYMBOL: &str = "☆";
pub const NO_STAR: &str = "";
pub const FULL_OPACITY: f64 = 1.0;
pub const HALF_OPACITY: f64 = 0.5;
pub const QUARTER_OPACITY: f64 = 0.1;
pub const MAX_LABEL_LENGTH: usize = 20;
pub const DIMENSION_MIN: i32 = 100;
pub const DIMENSION_MAX: i32 = 5000;
pub const SLIDESHOW_DELAY_MIN: i32 = 1;
pub const SLIDESHOW_DELAY_MAX: i32 = 900;
pub const DEFAULT_SLIDESHOW_DELAY: i32 = 60;
pub const MAX_PICTURES_PER_ROW: i32 = 10;
pub const FRAME_PALETTE_AREA_HEIGHT: i32 = 10;
pub const FRAME_PALETTE_AREA_WIDTH: i32 = 90;
pub const GRID_PALETTE_AREA_HEIGHT: i32 = 5;
pub const GRID_PALETTE_AREA_WIDTH: i32 = 60;

pub const EXPAND_ON_SYMBOL: &str = "  ⃞";
pub const FULL_SIZE_ON_SYMBOL: &str = " 🔍";
#[allow(dead_code)]
pub const MAX_PALETTE_COLORS: u8 = 10;
#[allow(dead_code)]
pub const ONE_PICTURE_PER_ROW: usize = 1;
#[allow(dead_code)]
pub const PALETTE_AREA_HEIGHT: i32 = 10;
#[allow(dead_code)]
pub const PALETTE_AREA_WIDTH: i32 = 90;
#[allow(dead_code)]
pub const SCROLL_STEP: f64 = 100.0;
pub const THUMB_SUFFIX: &str = "THUMB";
pub const VALID_EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "png", "JPG", "JPEG", "PNG"];
pub const GARBAGE: &str = "!:";
pub const APPLICATION_ID: &str = "org.example.gsr";
pub const CONFIG_FILE_DEFAULT: &str = ".gsr2.toml";
pub const CONFIG_FILE_VARIABLE: &str = "GSRCFG";
#[cfg(test)]
pub const TEST_DATABASE_FILE: &str = "testdata/gsr2.db";
