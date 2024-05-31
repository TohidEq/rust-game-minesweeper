use std::io::{self, stdout, Write};

use colors::Colors;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::{Print, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand,
};
use playground::{PlayerMoves, PlayerStatus, Playground};
use std::{thread, time};
pub mod colors;
pub mod config;
pub mod playground;

pub enum Status {
    New,
    Run,
    End,
    Pause,
    // Score,
    // Menu,
}

fn main() -> io::Result<()> {
    let mut sc = stdout();
    let mut status = Status::Run;
    let mut playground_frame = config::GAME_SPEED;
    let mut playground_time = 0;

    // clear screen
    sc.execute(terminal::Clear(terminal::ClearType::All))?;
    sc.execute(Hide)?;
    enable_raw_mode()?;

    let (max_x_fake, max_y_fake) = size()?;
    let (max_x, max_y): (u16, u16) = (
        max_x_fake - (config::MARGIN_LEFT * 2),
        max_y_fake - (config::MARGIN_TOP * 2),
    );

    let mut playground = Playground::new(max_x, max_y);
    playground.create_playground();
    playground.draw_playground(&mut sc);

    // main loog
    loop {
        playground_frame -= 1;
        match status {
            Status::New => {}
            Status::Run => {
                // ----
                // -> first draw
                // insert into playground

                playground.draw_playground(&mut sc);

                // playground.draw_score(&mut sc, &mut score, &score_bg);

                sc.flush()?;

                // ----
                // -> events
                // read and apply keyboard
                // `poll()` waits for an `Event` for a given time period
                if poll(time::Duration::from_millis(10))? {
                    // It's guaranteed that the `read()` won't block when the `poll()`
                    // function returns `true`
                    let key = read()?;

                    // clear the buffer
                    while poll(time::Duration::from_millis(10))? {
                        let _ = read();
                    }

                    match key {
                        Event::Key(event) => match event.code {
                            KeyCode::Char('q') => {
                                status = Status::End;
                            }

                            // disabled :)
                            /*
                            KeyCode::Char(' ') => {

                            }
                                    */
                            KeyCode::Char('w') => {
                                playground.player.next_move = PlayerMoves::Top;
                            }
                            KeyCode::Char('a') => {
                                playground.player.next_move = PlayerMoves::Left;
                            }
                            KeyCode::Char('s') => {
                                playground.player.next_move = PlayerMoves::Bottom;
                            }
                            KeyCode::Char('d') => {
                                playground.player.next_move = PlayerMoves::Right;
                            }
                            KeyCode::Char('e') => {
                                playground.player.next_move = PlayerMoves::Defuse;
                            }
                            KeyCode::Char('f') => {
                                playground.player.next_move = PlayerMoves::Flag;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                } else {
                    // Timeout expired and no `Event` is available
                }

                // ----
                // -> physics
                if playground_frame == 0 {
                    playground_time += 1;
                }

                playground.player_action();
                if playground.player.status == PlayerStatus::Dead {
                    status = Status::End;
                }

                // ---- end physics

                // ----
                // -> drawing
                playground.draw_status_bar(&mut sc, playground_time);
                playground.draw_playground(&mut sc);
                // playground.draw_border(&mut sc);
                sc.flush()?;
                // ----
            }
            Status::Pause => {}
            Status::End => {
                sc.execute(terminal::Clear(terminal::ClearType::All))?;

                let mut x = 1;
                let mut y = 0; // ▼
                let mut text = String::from("U LOST! :(((").white();

                text = Colors::fg_color(&config::STATUS_COLOR_FG, text);
                text = Colors::bg_color(&config::STATUS_COLOR_BG, text);

                sc.queue(MoveTo(x, y))
                    .unwrap()
                    .queue(Print(text.bold()))
                    .unwrap();
                sc.flush().unwrap();

                let millis = time::Duration::from_millis(800);
                thread::sleep(millis);

                // game is finished
                sc.execute(terminal::Clear(terminal::ClearType::All))?
                    .execute(Show)?;
                disable_raw_mode()?;
                break;
            }
        }
        if playground_frame == 0 {
            playground_frame = config::GAME_SPEED
        }
    }

    Ok(())
}
