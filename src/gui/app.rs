use dioxus_desktop::{WindowBuilder, LogicalSize};

use super::*;

pub fn start_gui() {
    let window = WindowBuilder::new()
        .with_title(format!("Pocus {}", PKG_VERSION))
        .with_inner_size(LogicalSize::new(1400, 1000));

    dioxus_desktop::launch_cfg(App, dioxus_desktop::Config::new().with_window(window));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Play,
    Tourney,
    Engines,
    Editor,
    Settings,
    Info,
}

fn App(cx: Scope) -> Element {
    let mut page = Page::Play;

    use_shared_state_provider(cx, || Page::Play);
    
    cx.render(rsx!{

        link { 
            href:"https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css",
            rel:"stylesheet"
        },

        Menu(cx),

        button { 
            onclick: move |_| println!("Dummy button clicked"),
            "Dummy button"
        },

        div {
            margin: "1em",
            match page {
                Page::Play => PlayPage(cx),
                Page::Tourney => TourneyPage(cx),
                _ => unreachable!()
            }
        }   
    })
}