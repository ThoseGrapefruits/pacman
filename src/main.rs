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

use ncurses::*;
use std::{env, cmp};
use character::{Character, Player, Ghost};

static DESIRED_LINES: i32 = 20;
static DESIRED_COLS: i32 = 32;
const ESC: i32 = 0x1B;
const CTRL_C: i32= 0x3;

/// Rust + ncurses-based console game
fn main() {
    // Initialize `ncurses`
    initscr();
    raw();
    keypad(stdscr, true);
    noecho();
    start_color();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let verbose: bool = env::args().any(|arg| {
        arg == "--verbose"
    });

    // Load `Player` and 4 `Ghost`s as `Character`s
    let player = Player::new();
    let ghosts = vec!(Ghost::new(), Ghost::new(),
                      Ghost::new(), Ghost::new());

    let characters = vec!(&player as &Character,
                          &ghosts[0], &ghosts[1],
                          &ghosts[2], &ghosts[3]);



    let window = create_centred_window();

    let mut ch = getch();

    // Main user input loop
    loop {
        mvprintw(LINES - 1, 0, &format!("{}, {}", ch, keyname(ch)));
        match ch {
            // Player movement
            KEY_LEFT => {
                // TODO
            },

            KEY_RIGHT => {
                // TODO
            },

            KEY_UP => {
                // TODO
            },

            KEY_DOWN => {
                // TODO
            },

            ESC => {
                mvprintw(0, 0, "Found escape");
            },

            KEY_F4 => {
                mvprintw(0, 0, "F4 Pressed");
            },

            CTRL_C => {
                break
            },

            _ => {
            },
        }
        refresh();
        ch = getch();
        clear();
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
