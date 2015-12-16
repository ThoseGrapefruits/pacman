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

use std::error::Error;

enum MenuObject {
    String,
    SubMenu,
    MainMenu,
    PauseMenu,
}

impl MenuObject {
    /// Get the parent menu of this menu
    fn get_parent(&self) -> MenuObject {

    }

    /// Move the cursor up one item
    fn cursor_up(&self) {

    }

    /// Move the cursor down one item
    fn cursor_down(&self) {

    }

    /// Select the current item, returning the `MenuObject`
    fn select(&self) -> MenuObject {

    }

    /// Opens selection and grabs user input until a selection has been made
    fn open_selection(&self, window: WINDOW) {
        loop {
            let input = wgetch();
            match input {}
        }
    }
}

pub struct MainMenu {
    index: u8,
    title: &str,
    items: Vec<MenuObject>,
}

impl MainMenu {
    fn new_with_title_and_items(title: &str, items: Vec<MenuObject>) -> MainMenu {
        MainMenu {
            index: 0,
            title: title,
            items: items,
        }
    }
}

pub struct PauseMenu {
    delegate: MainMenu,
}

pub struct SubMenu {
    delegate: MainMenu,
}

/// Builder for `Menu`s
pub struct MenuBuilder {
    index: u8,
    title: String,
    items: Vec<MenuObject>,
    kind: MenuObject,
}

pub impl MenuBuilder {
    fn new() -> Result<MenuBuilder, Error> {
        MenuBuilder::new_of_kind(MenuObject::MainMenu)
    }

    fn new_of_kind(kind: MenuObject) -> Result<MenuBuilder, Error> {
        match kind {
            MenuObject::MainMenu | MenuObject::PauseMenu | MenuObject::SubMenu => {
                Ok(MenuBuilder {
                    index: 0_u8,
                    title: "",
                    items: Vec::new(),
                    kind: kind,
                })
            },
            MenuObject::String => {
                Err("Cannot use MenuBuilder to build String");
            }
        }
    }
}
