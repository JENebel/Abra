use std::collections::HashMap;

use dioxus_desktop::{WindowBuilder, LogicalSize};
use dioxus_router::{Router, Route, Redirect};

use super::*;

pub fn start_gui() {
    let window = WindowBuilder::new()
        .with_title(format!("Abra {PKG_VERSION}"))
        .with_inner_size(LogicalSize::new(1600, 1000))
        .with_min_inner_size(LogicalSize::new(1200, 500));

    dioxus_desktop::launch_cfg(App, dioxus_desktop::Config::new().with_window(window));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Game,
    Tourney,
    Engines,
    Editor,
    Settings,
    About,
    Books,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedEngine {
    None,
    Engine(u32),
}

pub type EngineMap = HashMap<u32, Engine>;

pub fn RenderPage<'a>(cx: Scope<'a>, element: Element<'a>, page: Page) -> Element<'a> {
    let active_page = use_shared_state::<Page>(cx).unwrap();
    *active_page.write_silent() = page;

    cx.render(rsx!{
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
                element
            }
        }
    })
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::Tourney);
    use_shared_state_provider(cx, || load_all_engines().unwrap());
    use_shared_state_provider(cx, || SelectedEngine::None);
    
    cx.render(rsx!{
        link { 
            href:"https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css",
            rel:"stylesheet"
        },

        link { 
            href:"src/gui/style.css",
            rel:"stylesheet"
        },

        Router {
            Route { to: "/Game", RenderPage(cx, GamePage(cx), Page::Game) },
            Route { to: "/Tourney", RenderPage(cx, TourneyPage(cx), Page::Tourney) },
            Route { to: "/Engines", RenderPage(cx, EnginesPage(cx), Page::Engines) },

            Redirect { from: "", to: "/Game" }
        }
    })
}