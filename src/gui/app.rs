use std::collections::HashMap;

use dioxus_desktop::{WindowBuilder, LogicalSize};

use super::*;

pub fn start_gui() {
    let window = WindowBuilder::new()
        .with_title(format!("Pocus {PKG_VERSION}"))
        .with_inner_size(LogicalSize::new(1500, 1000))
        .with_min_inner_size(LogicalSize::new(1300, 600));

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

pub enum SelectedEngine {
    None,
    Engine(u32),
}

pub type EngineMap = HashMap<u32, Engine>;

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::Engines);
    use_shared_state_provider(cx, || load_all_engines().unwrap());
    use_shared_state_provider(cx, || SelectedEngine::None);
    let page = use_shared_state::<Page>(cx).unwrap();
    
    cx.render(rsx!{
        link { 
            href:"https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css",
            rel:"stylesheet"
        },

        link { 
            href:"src/gui/style.css",
            rel:"stylesheet"
        },

        body {
            style: r"
                display: flex;
                flex-flow: column;
                min-height: 100vh;
                max-height: 100vh;
                background-color: #ededed;
                box-sizing: border-box;
                margin: 0;
                padding: 0;
                overflow: hidden;
            ",
            
            div {
                style: r"
                    display: flex;
                    flex-flow: column;
                    flex: 1;
                    min-height: 100%;
                    max-height: 100%;
                    width: 100%;
                    margin: 0;
                    padding: 0;
                ",

                Menu(cx),
                match *page.read() {
                    Page::Play => Game(cx, true),
                    Page::Tourney => TourneyPage(cx),
                    Page::Engines => Engines(cx),
                    _ => render!{ p { "Not implemented yet" } }
                }
            }
        }
    })
}