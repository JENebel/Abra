#![allow(non_snake_case)]

pub use super::*;

pub use dioxus::prelude::*;

mod app;
mod menu;
mod game_page;
mod tourney_page;
mod board;
mod engines_page;
mod engine_info;

pub use app::*;
use menu::*;
use tourney_page::*;
use game_page::*;
use board::*;
use engines_page::*;
use engine_info::*;