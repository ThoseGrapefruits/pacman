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

use ncurses::WINDOW;

/// Trait specifying a struct which exists in 2D space
trait Location {

    /// Get x-coordinate of struct
    fn x(&self) -> i32;

    /// Get y-coordinate of struct
    fn y(&self) -> i32;
}

struct Point {
    
    /// X-coordinate
    x: i32,

    /// Y-coordinate
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) {
        Point {
            x: x,
            y: y,
        }
    }

    fn shift(&self, dx: i32, dy:i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Location for Point {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

/// Trait to be implemented for all structs to be rendered in `ncurses`
trait Visible : Location {

    /// Draw `self` on the given `window`
    fn draw(&self, window: WINDOW);
}

/// enum containing all moving characters (ghosts & player(s))

trait Character : Visible {

    fn new() -> Character;

    fn new_at_coords(x: i32, y: i32) -> Character;

    /// Shift `self` by the given `dx` and `dy`
    fn shift(&self, dx: i32, dy: i32);

    /// Shift `self` to (x, y)
    fn shift_to(&self, x: i32, y: i32);

    /// Shift `self` to the given `Point`
    fn shift_to_point(&self, point: Point) {
        self.shift_to(point.x(), point.y());
    }

    /// Get the next location to move to
    fn next(&self) -> Point;

    /// Go to the next location
    fn go_next(&self) {
        self.shift_to_point(self.next());
    }
}

pub struct Ghost {
    x: i32,
    y: i32,
}

impl Character for Ghost {
    /// Create a new `Ghost` at (0, 0)
    fn new() -> Ghost {
        Ghost::new_at_coords(0, 0)
    }

    /// Create a new `Ghost` at (x, y)
    fn new_at_coords(x: i32, y: i32) -> Ghost {
        Ghost {
            x: x,
            y: y,
        }
    }

    // TODO rest of `Character` `fn`s
    /// Shift `self` by the given `dx` and `dy`
    fn shift(&self, dx: i32, dy: i32);

    /// Shift `self` to (x, y)
    fn shift_to(&self, x: i32, y: i32);

    /// Shift `self` to the given `Point`
    fn shift_to_point(&self, point: Point) {
        self.shift_to(point.x(), point.y());
    }

    /// Get the next location to move to
    fn next(&self) -> Point;

    /// Go to the next location
    fn go_next(&self) {
        self.shift_to_point(self.next());
    }
}

impl Location for Ghost {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl Visible for Ghost {
    fn draw(&self, window: WINDOW) {
        // TODO
    }
}

pub struct Player {
    x: i32,
    y: i32,
}

impl Character for Player {
    /// Create a new player character at (0, 0)
    fn new() -> Player {
        Player::new_at_coords(0, 0);
    }

    /// Create a new player character at (x, y)
    fn new_at_coords(x: i32, y: i32) -> Player {
        Player {
            x: x,
            y: y,
        }
    }

    // TODO rest of `Character` `fn`s
    /// Shift `self` by the given `dx` and `dy`
    fn shift(&self, dx: i32, dy: i32);

    /// Shift `self` to (x, y)
    fn shift_to(&self, x: i32, y: i32);

    /// Shift `self` to the given `Point`
    fn shift_to_point(&self, point: Point) {
        self.shift_to(point.x(), point.y());
    }

    /// Get the next location to move to
    fn next(&self) -> Point;

    /// Go to the next location
    fn go_next(&self) {
        self.shift_to_point(self.next());
    }
}

impl Location for Player {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl Visible for Player {

    fn draw(&self, window: WINDOW) {
        // TODO
    }
}
