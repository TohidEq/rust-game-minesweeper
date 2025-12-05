use crate::colors;
use colors::Colors;

// u can change:
pub const GAME_SPEED: u16 = 10;
pub const EMPTY_CHAR: &str = "░";
pub const UNCLICKED_CHAR: &str = "▓";
pub const BOMB_CHAR: &str = "☠";
pub const FLAG_CHAR: &str = "󰈿";
// true = col and row will get color(selection) , false = only selected cell will get color
pub const SELECT_COL_ROW_OR_CELL: bool = false;

// 0: no bomb, 100: full bomb
pub const BOMB_RATE: u16 = 18;

// (%)   100% W == half screen
pub const PLAYGROUND_WITH: u16 = 100;
// (%)   100% H == full screen
pub const PLAYGROUND_HEIGHT: u16 = 100;

pub const MARGIN_TOP: u16 = 2;
pub const MARGIN_LEFT: u16 = 1;

// ====== colors ======
pub const PLAYER_SELECT_COLOR_FG: Colors = Colors::Black;
pub const PLAYER_SELECT_COLOR_BG: Colors = Colors::Yellow;

pub const STATUS_COLOR_FG: Colors = Colors::White;
pub const STATUS_COLOR_BG: Colors = Colors::Black;

pub const PLAYGROUND_COLOR_FG: Colors = Colors::White;
pub const PLAYGROUND_COLOR_BG: Colors = Colors::Black;

pub const PLAYER_DEFUSED_COLOR_FG: Colors = Colors::Black;
pub const PLAYER_DEFUSED_COLOR_BG: Colors = Colors::Black;

pub const PLAYER_FLAG_COLOR_FG: Colors = Colors::Black;
pub const PLAYER_FLAG_COLOR_BG: Colors = Colors::Green;

pub const BOMB_COLOR_FG: Colors = Colors::Black;
pub const BOMB_COLOR_BG: Colors = Colors::Red;

// ====== numbers colors ======
pub const BOMB_HELPER_COLOR_FG_1: Colors = Colors::Blue;
pub const BOMB_HELPER_COLOR_BG_1: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_2: Colors = Colors::Green;
pub const BOMB_HELPER_COLOR_BG_2: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_3: Colors = Colors::Red;
pub const BOMB_HELPER_COLOR_BG_3: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_4: Colors = Colors::DarkBlue;
pub const BOMB_HELPER_COLOR_BG_4: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_5: Colors = Colors::Brown;
pub const BOMB_HELPER_COLOR_BG_5: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_6: Colors = Colors::Cyan;
pub const BOMB_HELPER_COLOR_BG_6: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_7: Colors = Colors::Purple;
pub const BOMB_HELPER_COLOR_BG_7: Colors = Colors::Black;

pub const BOMB_HELPER_COLOR_FG_8: Colors = Colors::Grey;
pub const BOMB_HELPER_COLOR_BG_8: Colors = Colors::Black;
