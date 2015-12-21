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

use game_objects::{Player, Ghost, Coin, Visible, Character, Location};
use menu::{MenuBuilder, MenuObject};
use std::sync::{Arc, Mutex};

pub struct GameState {
    player: Arc<Mutex<Player>>,
    ghosts: Vec<Ghost>,
    coins: Vec<Coin>,
    paused: bool,
    menu: MenuObject,
}

impl GameState {
    /// Create a new GameState with the default map layout
    fn new() -> GameState {
        GameState {
            player: Arc::new(Mutex::new(Player::new())),
            ghosts: vec![Ghost::new(), Ghost::new(), Ghost::new(), Ghost::new()],
            coins: vec![Coin::new()], // TODO add rest of default coin positions
            paused: false,
            menu: MenuBuilder::new(), // TODO build menu
        }
    }

    /// Get the next state of the game
    fn next(&self) -> GameState {
        let player = self.player.lock().unwrap();

        // Move `Ghost`s forward to next step
        for ghost in self.ghosts {
            ghost.go_next();
        }

        // Move `Player` forward to next step
        player.go_next();

        for coin in self.coins {
            if coin.x() == player.x() && coin.y() == player.y() {

            }
        }
        // TODO rest of per-tick game updates

        *self
    }

    fn get_player(&self) -> Arc<Mutex<Player>> {
        self.player
    }

    // TODO rest of GameState functionality
}
