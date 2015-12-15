// Rusman - Rust + ncurses-based console game
// Copyright (C) 2015 Logan Moore
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
mod character;

extern crate ncurses;
extern crate clap;
extern crate toml;

use ncurses::*;
use std::{env, cmp, thread};
use character::{Player, Ghost};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use clap::{App, Arg};
use std::io::prelude::*;
use std::fs::File;
use toml::{Parser};

static DESIRED_LINES: i32 = 20;
static DESIRED_COLS: i32 = 32;
const ESC: i32 = 0x1B;
const CTRL_C: i32= 0x3;

/// Rust + ncurses-based console game
fn main() {
    // Command line argument parsing
    let cwd = env::current_dir().expect("could not open cwd");
    let mut cargo_toml = File::open(cwd.join("Cargo.toml")).expect("could not open file");
    let mut s = String::new();
    cargo_toml.read_to_string(&mut s).expect("Could not read file to string");
    let cargo_toml_parsed = Parser::new(&s).parse().expect("Could not parse file into TOML");
    let package = cargo_toml_parsed.get("package").expect("could not find [package]");

    let package_table = package.as_table().expect("not a table");
    println!("{:?}", package_table);
    let name = package_table.get("name").expect("could not find name")
        .as_str().expect("name not a string");
    let version = package_table.get("version").expect("could not find version")
        .as_str().expect("version not a string");
    let authors = package_table.get("authors").expect("could not find authors")
        .as_slice().expect("not an array");
    let description = package_table.get("description").expect("could not find description")
        .as_str().expect("description not a string");

    let mut app = App::new(name).version(version);

    for author in authors {
        app = app.author(author.as_str().expect("author not a string"));
    }

    let matches = app.about(description)
            .arg(Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .get_matches();
    let verbose = matches.occurrences_of("verbose");

    // Initialize `ncurses`
    initscr();
    raw();
    keypad(stdscr, true);
    noecho();
    start_color();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Load `Player` and 4 `Ghost`s as `Character`s
    let player = Arc::new(Mutex::new(Player::new()));
    let ghosts = Arc::new(Mutex::new(vec!(Ghost::new(), Ghost::new(),
                                          Ghost::new(), Ghost::new())));
    let window = create_centred_window();

    let (tx, rx) = mpsc::channel::<KeyResponse>();


    thread::spawn(move || -> KeyResponse {
        loop {
            tx.send(respond_to_key(getch())).unwrap();
        }
    });

    loop {
        match rx.recv().unwrap() {
            KeyResponse::Quit => {
                break
            }
            KeyResponse::Move(direction) => {
                match direction {
                    Direction::Up => {
                        // TODO
                        mvprintw(1, COLS - 1 - 25, "Move Up Rcvd");
                    }

                    Direction::Down => {
                        // TODO
                        mvprintw(1, COLS - 1 - 25, "Move Down Rcvd");
                    }

                    Direction::Left => {
                        // TODO
                        mvprintw(1, COLS - 1 - 25, "Move Left Rcvd");
                    }

                    Direction::Right => {
                        // TODO
                        mvprintw(1, COLS - 1 - 25, "Move Right Rcvd");
                    }
                }
            }
            KeyResponse::Pause => {

            }
            KeyResponse::Void => {

            }
        }
    }


    // Stop `ncurses`
    destroy_window(window);
    endwin();
}

/// Open a new, boxed ncurses window
fn create_window(y: i32, x: i32,
                 lines: i32, cols: i32) -> WINDOW {
    let window = newwin(lines, cols, y, x);
    box_(window, 0, 0);
    wrefresh(window);

    window
}

fn create_centred_window() -> WINDOW {
    let (max_lines, max_cols) = get_max_bounds(stdscr);
    let lines: i32 = cmp::min(DESIRED_LINES, max_lines);
    let cols: i32 = cmp::min(DESIRED_COLS, max_cols);

    create_window((max_lines - lines) / 2, (max_cols - cols) / 2,
                  lines, cols)
}

/// Destroy an ncurses window
fn destroy_window(window: WINDOW) {
    let ch = ' ' as chtype;
    wborder(window, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(window);
    delwin(window);
}

/// Get the maximum bounds of the console
fn get_max_bounds(window: WINDOW) -> (i32, i32) {
    let (mut max_y, mut max_x) = (0, 0);
    getmaxyx(window, &mut max_y, &mut max_x);

    (max_y, max_x)
}

enum KeyResponse {
    Quit,
    Move(Direction),
    Pause,
    Void,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn respond_to_key(ch: i32) -> KeyResponse {
        mvprintw(LINES - 1, 0, &format!("{}, {}", ch, keyname(ch)));
        match ch {
            // Player movement
            KEY_LEFT => {
                // TODO
                mvprintw(0, COLS - 1 - 20, "Move Left Sent");
                KeyResponse::Move(Direction::Left)
            },

            KEY_RIGHT => {
                // TODO
                mvprintw(0, COLS - 1 - 20, "Move Right Sent");
                KeyResponse::Move(Direction::Right)
            },

            KEY_UP => {
                // TODO
                mvprintw(0, COLS - 1 - 20, "Move Up Sent");
                KeyResponse::Move(Direction::Up)
            },

            KEY_DOWN => {
                // TODO
                mvprintw(0, COLS - 1 - 20, "Move Down Sent");
                KeyResponse::Move(Direction::Down)
            },

            ESC => {
                mvprintw(0, 0, "Found escape");
                KeyResponse::Pause
            },

            KEY_F4 => {
                mvprintw(0, 0, "F4 Pressed");
                KeyResponse::Pause
            },

            CTRL_C => {
                KeyResponse::Quit
            },

            _ => {
                KeyResponse::Void
            },
        }
}
