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

extern crate ncurses;

use ncurses::*;
use std::fmt;
use std::env;

static DESIRED_WINDOW_WIDTH: i32 = 32;
static DESIRED_WINDOW_HEIGHT: i32 = 20;
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

    let (max_y, max_x) = get_max_bounds(stdscr);

    let window = create_window(0, 0);

    let mut ch = getch();

    if verbose {
        printw("verbose");
    }

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

            ERR => {
                printw("ERR");
            },

            ESC => {
                printw("Found escape");
                break
            },

            KEY_F4 => {
                printw("F4 Pressed");
                break
            },

            KEY_CLOSE => {
                break
            }

            CTRL_C => {
                break
            }

            _ => {
            },
        }
        ch = getch();
    }

    // Stop `ncurses`
    destroy_window(window);
    endwin();
}

/// Open a new, boxed ncurses window
fn create_window(y: i32, x: i32) -> WINDOW {
    let window = newwin(DESIRED_WINDOW_WIDTH, DESIRED_WINDOW_HEIGHT, y, x);
    box_(window, 0, 0);
    wrefresh(window);
    window
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
