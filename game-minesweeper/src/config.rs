use crate::colors;
use colors::Colors;

// u can change:
pub const EMPTY_CHAR: &str = " ";
pub const BOMB_CHAR: &str = "X";
pub const FLAG_CHAR: &str = "F";

// 0: no bomb, 100: full bomb
pub const BOMB_RATE: u16 = 50;

// (%)   100% W == half screen
pub const PLAYGROUND_WITH: u16 = 100;
// (%)   100% H == full screen
pub const PLAYGROUND_HEIGHT: u16 = 100;

pub const PLAYER_SELECT_COLOR_BG: Colors = Colors::Black;
pub const PLAYER_SELECT_COLOR_FG: Colors = Colors::Yellow;

pub const PLAYGROUND_COLOR_BG: Colors = Colors::Black;
pub const PLAYGROUND_COLOR_FG: Colors = Colors::Black;
