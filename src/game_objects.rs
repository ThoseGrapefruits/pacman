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

use ncurses::{WINDOW, mvwprintw};
use std::cmp::{Eq, PartialEq};

/// Trait specifying a struct which exists in 2D space
trait Location {
    /// Get x-coordinate of struct
    fn x(&self) -> i32;

    /// Get y-coordinate of struct
    fn y(&self) -> i32;
}

/// A point in 2D space
struct Point {
    /// X-coordinate
    x: i32,

    /// Y-coordinate
    y: i32,
}

impl Point {
    /// Create a new point with the given coordinates
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    /// Shift this point by the given deltas
    fn shift(&self, dx: i32, dy:i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// Shift this point to the given coordinates
    fn shift_to(&self, x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

impl Eq for Location {}

impl Location for Point {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

/// Trait to be implemented for all structs to be rendered in `ncurses`
pub trait Visible {
    /// Draw `self` on the given `window`
    fn draw(&self, window: WINDOW);
}

pub trait Character : Visible {

    /// Shift `self` by the given `dx` and `dy`
    fn shift(&mut self, dx: i32, dy: i32);

    /// Shift `self` to (x, y)
    fn shift_to(&mut self, x: i32, y: i32);

    /// Shift `self` to the given `Point`
    fn shift_to_point(&mut self, point: Point) {
        self.shift_to(point.x(), point.y());
    }

    /// Get the next location to move to
    fn next(&self) -> Point;

    /// Go to the next location
    fn go_next(&mut self) {
        let next: Point = self.next();
        self.shift_to_point(next);
    }
}

pub struct Coin {
    coords: Point,
    big: bool,
}

impl Coin {
    pub fn new() -> Coin {
        Coin::new_at_coords(0, 0, false)
    }

    pub fn new_at_coords(x: i32, y: i32, big: bool) -> Coin {
        Coin {
            coords: Point::new(x, y),
            big: big,
        }
    }

    pub fn new_at_point(coords: Point, big: bool) -> Coin {
        Coin {
            coords: coords,
            big: big,
        }
    }

    /// Check if the given `Coin` is big (gives more points if so)
    pub fn is_big(&self) -> bool {
        self.big
    }
}


impl Visible for Coin {
    fn draw(&self, window: WINDOW) {
        if self.is_big() {
            try_draw_utf8(window, vec![0xE2, 0x97, 0x89], self.x(), self.y());
        } else {
            try_draw_utf8(window, vec![0xE2, 0x97, 0x8F], self.x(), self.y());
        }
    }
}

impl Location for Coin {
    /// Get x-coordinate of struct
    fn x(&self) -> i32 {
        self.coords.x()
    }

    /// Get y-coordinate of struct
    fn y(&self) -> i32 {
        self.coords.y()
    }
}

pub struct Ghost {
    coords: Point,
}

unsafe impl Sync for Ghost {}

impl Ghost {
    /// Create a new `Ghost` at (0, 0)
    pub fn new() -> Ghost {
        Ghost::new_at_coords(0, 0)
    }

    /// Create a new `Ghost` at (x, y)
    pub fn new_at_coords(x: i32, y: i32) -> Ghost {
        Ghost {
            coords: Point::new(x, y),
        }
    }
}

impl Character for Ghost {

    // TODO rest of `Character` `fn`s
    /// Shift `self` by the given `dx` and `dy`
    fn shift(&mut self, dx: i32, dy: i32) {
        self.coords = self.coords.shift(dx, dy);
    }

    /// Shift `self` to (x, y)
    fn shift_to(&mut self, x: i32, y: i32) {
        self.coords = self.coords.shift_to(x, y);
    }

    /// Get the next location to move to
    fn next(&self) -> Point {
        // TODO
        Point::new(self.coords.x(), self.coords.y())
    }

    pub fn go_next(&mut self) {
        // TODO
    }
}

impl Location for Ghost {
    fn x(&self) -> i32 {
        self.coords.x()
    }

    fn y(&self) -> i32 {
        self.coords.y()
    }
}

impl Visible for Ghost {
    /// Draw the ghost emoji at this `Ghost`'s coordinates
    fn draw(&self, window: WINDOW) {
        try_draw_utf8(window, vec![0xF0, 0x9F, 0x91, 0xBB], self.x(), self.y());
    }
}

pub fn try_draw_utf8(window: WINDOW, vec: Vec<u8>, x: i32, y: i32) {
    let to_draw = String::from_utf8(vec![0xF0, 0x9F, 0x91, 0xBB]);

    // Print ghost emoji
    if to_draw.is_err() {
        mvwprintw(window, x, y, "!");
    } else {
        mvwprintw(window, x, y, &to_draw.unwrap());
    }
}

pub struct Player {
    coords: Point,
}

unsafe impl Sync for Player {}

impl Player {
    /// Create a new player character at (0, 0)
    pub fn new() -> Player {
        Player::new_at_coords(0, 0)
    }

    /// Create a new player character at (x, y)
    fn new_at_coords(x: i32, y: i32) -> Player {
        Player {
            coords: Point::new(x, y),
        }
    }
}

#[test]
pub fn test_player() {
    let mut player = Player::new_at_coords(10, 10);
    assert_eq!(player.x(), 10);
    assert_eq!(player.y(), 10);

    player.shift(10, 10);
    assert_eq!(player.x(), 20);
    assert_eq!(player.y(), 20);

    player.shift_to(27, 34);
    assert_eq!(player.x(), 27);
    assert_eq!(player.y(), 34);

    player.shift_to_point(Point {x: 9, y: 11});
    assert_eq!(player.x(), 9);
    assert_eq!(player.y(), 11);
}

impl Character for Player {

    // TODO rest of `Character` `fn`s
    /// Shift `self` by the given `dx` and `dy`
    fn shift(&mut self, dx: i32, dy: i32) {
        self.coords.shift(dx, dy);
    }

    /// Shift `self` to (x, y)
    fn shift_to(&mut self, x: i32, y: i32) {
        self.coords.shift_to(x, y);
    }

    /// Get the next location to move to
    fn next(&self) -> Point {
        // TODO
        Point{x: self.x(), y: self.y()}
    }

    pub fn go_next(&mut self) {
        // TODO
    }
}

impl Location for Player {
    fn x(&self) -> i32 {
        self.coords.x()
    }

    fn y(&self) -> i32 {
        self.coords.y()
    }
}

impl Visible for Player {
    fn draw(&self, window: WINDOW) {
        try_draw_utf8(window, vec![0xF0, 0x9F, 0x91, 0xBB], self.x(), self.y());
    }
}
