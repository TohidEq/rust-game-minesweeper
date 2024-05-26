// NOTE remove derive"Debug" later

use crate::colors;
use crate::config;
use colors::Colors;

use crossterm::style::Print;
use rand::{random, Rng};

// enums
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellStatus {
    Bomb, // BOOM
    Safe, // 0~8
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ClickStatus {
    Not,
    Defusing,
    Flag,
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
#[derive(Debug)]
pub struct Cell {
    pub(crate) status: CellStatus,
    pub(crate) click: ClickStatus,
    pub(crate) location: Location,
    // pub(crate) color: Color,
}

pub struct Playground {
    pub screen_width: u16,
    pub screen_height: u16,
    pub width: u16,
    pub height: u16,

    pub cells: Vec<Cell>,
    // pub cells: Vec<(u16, u16)>,
}

// happy happy happy
impl Playground {
    pub fn new(max_x: u16, max_y: u16) -> Playground {
        return Playground {
            screen_width: max_x,
            screen_height: max_y,
            width: 0,
            height: 0,
            cells: vec![],
        };
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
                    click: ClickStatus::Not,
                    location: Location { x, y },
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
                let new_cell = Cell {
                    status: CellStatus::Bomb,
                    click: cell.click.clone(),
                    location: cell.location.clone(),
                };

                self.update_cell(new_cell);
                bombs_planted += 1;
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

    pub fn update_cell(&mut self, cell: Cell) {
        let x = cell.location.x;
        let y = cell.location.y;
        let index = self.get_index(x, y);

        self.cells[index] = cell;
    }
}
