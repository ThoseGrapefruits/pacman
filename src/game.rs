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

pub struct GameState {
    player: Player,
    ghosts: Vec<Ghost>,
    paused: bool,
    menu: MenuObject
}

impl GameState {
    /// Create a new GameState
    fn new() {
        GameState {
            player: Player::new(),
            ghosts: vec![Ghost::new(), Ghost::new(), Ghost::new(), Ghost::new()],
            paused: false,
            menu: MainMenu::new(),
        }
    }

    /// Get the next state of the game
    fn next(&self) {

    }

    fn get_player(&self) -> Player {
        self.player
    }

    // TODO rest of GameState functionality
}
