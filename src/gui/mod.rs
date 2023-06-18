#![allow(non_snake_case)]

pub use super::*;

pub use dioxus::prelude::*;

mod app;
mod menu;
mod game;
mod tourney_page;
mod board;

pub use app::*;
use menu::*;
use tourney_page::*;
use game::*;
use board::*;