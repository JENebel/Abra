use std::collections::HashMap;

use super::*;

fn EngineListItem<'a>(cx: Scope<'a>, engine: Engine, engines: &'a UseSharedState<HashMap<u32, Engine>>) -> Element<'a> {
    let selected_engine = use_shared_state::<SelectedEngine>(cx).unwrap();

    let background = if let SelectedEngine::Engine(id) = *selected_engine.read() {
        if id == engine.id {
            "background-color: #bfbfbf;"
        } else {
            ""
        }
    } else {
        ""
    };

    cx.render(rsx!{
        div {
            class: "pure-button",
            style: r"
                display: flex;
                flex-flow: row;
                justify-content: space-between;
                margin: 0;
                font-size: 1.5em;
                height: 2.2em;
                {background}
                cursor: context-menu;
            ",

            onclick: move |_event| { 
                *selected_engine.write() = SelectedEngine::Engine(engine.id);
            },

            p { 
                style: "align-self: flex-start; margin: 0;",
                "{engine.alias}"
            },

            p {
                style: "align-self: flex-end; font-weight: bold; color: red; margin: 0; cursor: pointer;",

                onclick: move |_event| {
                    remove_engine(engine.id).unwrap();
                    engines.write().remove(&engine.id);
                    *selected_engine.write() = SelectedEngine::None;
                },

                "X"
            }
        }
    })
}

pub fn Engines(cx: Scope) -> Element {
    let engines = use_shared_state::<HashMap<u32, Engine>>(cx).unwrap();

    cx.render(rsx!{
        // Container
        div {
            style: r"
                display: flex;
                flex-flow: row;
                height: 100%;
                flex: 1;
                background-color: #A4A6A5;
                overflow: hidden;
            ",

            // Engine list
            div {
                style: "
                    height: 100%;
                    min-width: 35em;
                ",

                div {
                    style: "display: flex; flex-direction: row; justify-content: center;",
                    button {
                        style: "margin-top: 1.5em;",
                        class: "pure-button",

                        onclick: move |_event| { 
                            let mut engine = EngineWrapper::get_info("C:/Users/Joachim/VSCode Projects/Cadabra/target/release/cadabra.exe".to_string()).unwrap();
                            loop {
                                engine.id = rand::random::<u32>();
                                if !engines.read().contains_key(&engine.id) {
                                    break;
                                }
                            }
                            store_engine(&engine).unwrap();
                            (engines.write()).insert(engine.id, engine);
                        },
                        "Install new"
                    },
                },

                // The list
                div {
                    style: r"
                        display: flex;
                        flex-flow: column;
                        height: 100%;
                        flex: 1;
                        background-color: #A4A6A5;
                        margin: 1em;
                    ",

                    for engine in engines.read().values() {
                        EngineListItem(cx, engine.clone(), engines)
                    }
                }
            },
            // Engine info & settings
            div {
                class: "box",
            }
        }
    })
}