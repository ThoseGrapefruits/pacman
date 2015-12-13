extern crate ncurses;

use ncurses::*;

static WINDOW_WIDTH: i32 = 32;
static WINDOW_HEIGHT: i32 = 20;

/// Console-based pacman
fn main() {
    // Initialize `ncurses`
    initscr();

    let window = create_window(0, 0);

    let mut ch = getch();
    while ch != KEY_F(4) {
        
        ch = getch();
    }

    // Stop `ncurses`
    destroy_window(window);
    endwin();
}

fn create_window(y: i32, x: i32) -> WINDOW {
    let window = newwin(WINDOW_WIDTH, WINDOW_HEIGHT, y, x);
    box_(window, 0, 0);
    wrefresh(window);
    window
}

fn destroy_window(window: WINDOW) {
    let ch = ' ' as chtype;
    wborder(window, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(window);
    delwin(window);
}
