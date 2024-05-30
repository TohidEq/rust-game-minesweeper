// NOTE remove derive"Debug" later

use crate::colors;
use crate::config;
use colors::Colors;
use crossterm::style::Print;
use rand::seq::index;
use rand::{random, Rng};
use std::io::Stdout;

// enums
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellStatus {
    Bomb, // BOOM
    Safe, // 0~8
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ClickStatus {
    Noting,
    Defused,
    Flaged,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PlayerMoves {
    Stay, // nothing

    Top,
    Right,
    Bottom,
    Left,

    // Defuse {E}
    Defuse,
    // Flag {Q}
    Flag,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PlayerStatus {
    Alive,
    Dead,
    Victory,
}

// structs

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub x: u16,
    pub y: u16,
}
// no colors yet, just logic
// pub struct Color {
//     pub(crate) fg_color: Colors,
//     pub(crate) bg_color: Colors,
// }
#[derive(Debug, Clone)]
pub struct Cell {
    pub(crate) status: CellStatus,
    pub(crate) click: ClickStatus,
    pub(crate) location: Location,
    // pub(crate) color: Color,
    pub(crate) bomb_helper: u16, // bombs number around the cell
}

pub struct Player {
    pub status: PlayerStatus,
    pub location: Location,
    pub next_move: PlayerMoves,
}

pub struct Playground {
    pub screen_width: u16,
    pub screen_height: u16,
    pub width: u16,
    pub height: u16,

    pub cells: Vec<Cell>,

    pub player: Player,
    // pub cells: Vec<(u16, u16)>,
}

// happy happy happy
impl Playground {
    pub fn new(max_x: u16, max_y: u16) -> Playground {
        let player = Player {
            location: Location { x: 1, y: 1 },
            next_move: PlayerMoves::Stay,
            status: PlayerStatus::Alive,
        };
        return Playground {
            screen_width: max_x,
            screen_height: max_y,
            width: 0,
            height: 0,
            cells: vec![],
            player,
        };
    }
    pub fn number_of_bombs_around_cell(&mut self, _x: u16, _y: u16) -> u16 {
        let mut bombs_count = 0;

        let mut x: i16 = _x as i16;
        let mut y: i16 = _y as i16;

        // right
        x = _x as i16 + 1;
        y = _y as i16;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }

        //bottom
        x = _x as i16;
        y = _y as i16 + 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        //left
        x = _x as i16 - 1;
        y = _y as i16;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        //top
        x = _x as i16;
        y = _y as i16 - 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        //right bottom
        x = _x as i16 + 1;
        y = _y as i16 + 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        //right top
        x = _x as i16 + 1;
        y = _y as i16 - 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        // left bottom
        x = _x as i16 - 1;
        y = _y as i16 + 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }
        //left top
        x = _x as i16 - 1;
        y = _y as i16 - 1;
        if self.in_playground_range(x, y) {
            if self.cell_has_bomb(x as u16, y as u16) {
                bombs_count += 1;
            }
        }

        return bombs_count;
    }

    pub fn cell_has_bomb(&mut self, x: u16, y: u16) -> bool {
        let index = self.get_index(x, y);
        let cell = self.get_cell(index);
        if cell.status == CellStatus::Bomb {
            return true;
        }
        return false;
    }

    pub fn create_playground(&mut self) {
        let _x = self.screen_width * config::PLAYGROUND_WITH / 100; // - 2;
        let _y = self.screen_height * config::PLAYGROUND_HEIGHT / 100; // - 2;
        self.width = _x;
        self.height = _y;
        self.cells = vec![];

        for y in 0.._y {
            for x in 0.._x {
                self.cells.push(Cell {
                    status: CellStatus::Safe,
                    click: ClickStatus::Noting,
                    location: Location { x, y },
                    bomb_helper: 0,
                })
            }
        }

        // add bombs
        let number_of_bombs: u16 = (config::BOMB_RATE * self.cells.len() as u16) / 100;
        let mut bombs_planted = 0;

        while bombs_planted != number_of_bombs {
            let random_index = rand::thread_rng().gen_range(0..self.cells.len());
            let cell = self.get_cell(random_index);

            if cell.status == CellStatus::Safe {
                self.cells[random_index].status = CellStatus::Bomb;
                bombs_planted += 1;
            }
        }

        // calculate and add bomb helper
        for index in 0..self.cells.len() {
            let cell = self.get_cell(index);
            // remove this {if} if u want te create bomb_helper for all cells
            if cell.status == CellStatus::Safe {
                let x = cell.location.x;
                let y = cell.location.y;
                let bomb_numbers = self.number_of_bombs_around_cell(x, y);
                self.cells[index].bomb_helper = bomb_numbers;
            }
        }

        // LOG
        println!("cells: {:?}", self.cells);
    }

    pub fn get_index(&mut self, _x: u16, _y: u16) -> usize {
        let index = _y * (self.width) + _x;
        return index as usize;
    }

    pub fn get_cell(&mut self, index: usize) -> &Cell {
        return &self.cells[index];
    }

    // myb we can delete this... myb...
    pub fn update_cell(&mut self, cell: Cell) {
        let x = cell.location.x;
        let y = cell.location.y;
        let index = self.get_index(x, y);

        self.cells[index] = cell;
    }

    pub fn in_playground_range(&mut self, x: i16, y: i16) -> bool {
        //check NOT out of playground range
        let top_correct = y >= 0;
        let right_correct = x < self.width as i16;
        let botttom_correct = y < self.height as i16;
        let left_correct = x >= 0;
        let check_all = top_correct && right_correct && botttom_correct && left_correct;
        return check_all;
    }

    pub fn move_player(&mut self, _x: i16, _y: i16) {
        let x = self.player.location.x as i16 + _x;
        let y = self.player.location.y as i16 + _y;

        //check it is inside playground range
        if self.in_playground_range(x, y) {
            // move player
            self.player.location.x = x as u16;
            self.player.location.y = y as u16;
        }
    }

    pub fn player_action(&mut self) {
        match self.player.next_move {
            PlayerMoves::Stay => {}

            PlayerMoves::Top => self.move_player(0, -1),
            PlayerMoves::Right => self.move_player(1, 0),
            PlayerMoves::Bottom => self.move_player(0, 1),
            PlayerMoves::Left => self.move_player(-1, 0),

            PlayerMoves::Defuse => self.defuse_action(),
            PlayerMoves::Flag => self.flag_toggle_action(),
        }
        // after doing player action, we stay afk until next action by player
        self.player.next_move = PlayerMoves::Stay;
    }

    // player selected cell -> defuse
    pub fn defuse_action(&mut self) {
        let cell_index = self.get_index(self.player.location.x, self.player.location.y);
        let defusing_cell = self.get_cell(cell_index);

        /*
        NOTE:
            write:
                user can defuse cells that are defused and bombs flagged around them
        */

        if defusing_cell.status == CellStatus::Safe {
            // log defused
            self.cells[cell_index].click = ClickStatus::Defused;
        } else {
            // not defused, log u r dead...
            self.player.status = PlayerStatus::Dead;
        }
    }

    // player selected cell -> (flaged / unflaged)
    pub fn flag_toggle_action(&mut self) {
        let cell_index = self.get_index(self.player.location.x, self.player.location.y);
        let flagging_cell = self.get_cell(cell_index);
        let check_cell_action_is_noting = flagging_cell.click == ClickStatus::Noting;

        self.cells[cell_index].click = if check_cell_action_is_noting {
            ClickStatus::Flaged // cell flaged
        } else {
            ClickStatus::Noting // cell unflaged
        };
    }

    pub fn defuse_cell(&mut self, _x: u16, _y: u16) -> bool {
        let cell_index = self.get_index(_x, _y);
        let defusing_cell = self.get_cell(cell_index);

        if defusing_cell.status == CellStatus::Safe && defusing_cell.click == ClickStatus::Noting {
            self.cells[cell_index].click = ClickStatus::Defused;
            return true; // defused succesful
        }

        return false; // not defused
    }
}
