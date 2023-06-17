#![allow(non_snake_case)]

pub use super::*;

pub use dioxus::prelude::*;

mod app;
mod menu;
mod play_page;
mod tourney_page;

pub use app::*;
use menu::*;
use play_page::*;
use tourney_page::*;