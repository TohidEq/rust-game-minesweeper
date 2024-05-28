// NOTE remove derive"Debug" later

use crate::colors;
use crate::config;
use colors::Colors;
use crossterm::style::Print;
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
    Not,
    Defusing,
    Flag,
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
}
pub struct Player {
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
            next_move:PlayerMoves::
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
                let mut new_cell = cell.clone();
                new_cell.status = CellStatus::Bomb;

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

    pub fn move_player(&mut self,_x:i16,_y:i16){
        let x = self.player.location.x as i16+_x;
        let y = self.player.location.y as i16+_y;
        //check NOT out of playground range
        let top_correct=y>=0;
        let right_correct = x<self.width as i16;
        let botttom_correct = y<self.height as i16;
        let left_correct = x>=0;
        let check_all = top_correct && right_correct && botttom_correct && left_correct;
        if check_all{
            // move player
            self.player.location.x = x as u16;
            self.player.location.y = y as u16;
        }
    }

    pub fn player_action(&mut self){
        match self.player.next_move {
            PlayerMoves::Stay=>{}

            PlayerMoves::Top=>{self.move_player(0, -1)}
            PlayerMoves::Right=>{self.move_player(1, 0)}
            PlayerMoves::Bottom =>{self.move_player(0, 1)}
            PlayerMoves::Left=>{self.move_player(-1, 0)}

            PlayerMoves::Defuse=>{self.defuse_cell()}
            PlayerMoves::Flag=>{self.flag_cell()}
        }
        // after doing player action, we stay afk until next action by player
        self.player.next_move=PlayerMoves::Stay;
    }


    pub fn defuse_cell(&mut self){}
    pub fn flag_cell(&mut self){}
    
}
