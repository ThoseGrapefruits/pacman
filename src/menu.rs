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

use std::error::Error;
use ncurses::{wgetch, WINDOW};


enum MenuObject {
    String,
    SubMenu,
    MainMenu,
    PauseMenu,
}

impl MenuObject {
    /// Opens selection and grabs user input until a selection has been made
    fn open_selection(&self, window: ncurses::WINDOW) {
        loop {
            let input = ncurses::wgetch(window);
            match input {
                // TODO
            }
        }
    }
}

pub struct MainMenu {
    index: u8,
    title: String,
    items: Vec<MenuObject>,
    parent: Option<MenuObject>,
}

impl MainMenu {
    fn new_with_title_and_items(title: &str, items: Vec<MenuObject>) -> MainMenu {
        MainMenu {
            index: 0,
            title: String::from(title),
            items: items,
            parent: None,
        }
    }

    /// Get the parent menu of this menu
    fn get_parent(&self) -> Option<MenuObject> {
        self.parent
    }

    /// Move the cursor up one item
    fn cursor_up(&self) {
        // TODO
    }

    /// Move the cursor down one item
    fn cursor_down(&self) {
        // TODO
    }

    /// Select the current item, returning the `MenuObject`
    fn select(&self) -> MenuObject {
        // TODO
    }
}

pub struct PauseMenu {
    delegate: MainMenu,
}

impl PauseMenu {

    /// Get the parent menu of this menu
    fn get_parent(&self) -> Option<MenuObject> {
        self.delegate.get_parent()
    }

    /// Move the cursor up one item
    fn cursor_up(&self) {
        self.delegate.cursor_up()
    }

    /// Move the cursor down one item
    fn cursor_down(&self) {
        self.delegate.cursor_down()
    }

    /// Select the current item, returning the `MenuObject`
    fn select(&self) -> MenuObject {
        self.delegate.select()
    }
}

pub struct SubMenu {
    delegate: MainMenu,
}

impl SubMenu {
    /// Get the parent menu of this menu
    fn get_parent(&self) -> Option<MenuObject> {
        self.delegate.get_parent()
    }

    /// Move the cursor up one item
    fn cursor_up(&self) {
        self.delegate.cursor_up()
    }

    /// Move the cursor down one item
    fn cursor_down(&self) {
        self.delegate.cursor_down()
    }

    /// Select the current item, returning the `MenuObject`
    fn select(&self) -> MenuObject {
        self.delegate.select()
    }
}

/// Builder for `Menu`s
pub struct MenuBuilder<'a> {
    index: u8,
    title: &'a str,
    items: Vec<MenuObject>,
    kind: MenuObject,
}

impl <'a> MenuBuilder<'a> {
    /// Create a new `MenuBuilder` to build a `MainMenu`
    pub fn new() -> Result<MenuBuilder<'a>, &'a str> {
        MenuBuilder::new_of_kind(MenuObject::MainMenu)
    }

    /// Create a new `MenuBuilder` to build a `MenuObject` of the designated kind
    pub fn new_of_kind(kind: MenuObject) -> Result<MenuBuilder<'a>, &'a str> {
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
                Err("Cannot use MenuBuilder to build String")
            }
        }
    }

    fn index(&self, index: u8) {
        self.index = index;
    }

    fn title(&self, title: &'a str) {
        self.title = title;
    }

    fn items(&self, items: Vec<MenuObject>) {
        self.items = items;
    }

    fn kind(&self, kind: MenuObject) {
        self.kind = kind;
    }
}
