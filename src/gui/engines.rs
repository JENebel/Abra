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
                font-size: 1.5em;
                height: 2.2em;
                {background}
                cursor: context-menu;
                overflow: hidden;
                margin: 0.5;
                margin-bottom: 0.21em;
            ",

            onclick: move |_event| { 
                *selected_engine.write() = SelectedEngine::Engine(engine.id);
            },

            p { 
                style: "align-self: flex-start; margin: 0; font-weight: bold;",
                "{engine.alias}"
            },

            div {
                style: "self-align: flex-end; display: flex; flex-flow: row;",

                p {
                    style: r"
                        font-size: 0.5em;
                        text-align: center;
                        margin-right: 2em;
                        margin-top: 0.5em;
                        height: 100%;
                    ",

                    "{engine.name}"
                }

                p {
                    class: "x",
                    style: r"
                        margin: 0;
                    ",

                    onclick: move |event| {
                        event.stop_propagation();
                        remove_engine(engine.id).unwrap();
                        engines.write().remove(&engine.id);
                        *selected_engine.write() = SelectedEngine::None;
                    },

                    "X"
                }
            }
        }
    })
}

pub fn Engines(cx: Scope) -> Element {
    let engines = use_shared_state::<HashMap<u32, Engine>>(cx).unwrap();

    let mut sorted_engines: Vec<Engine> = engines.read().values().cloned().collect::<Vec<Engine>>();
    sorted_engines.sort_by(|a, b| a.alias.cmp(&b.alias));

    cx.render(rsx!{
        // Container
        div {
            style: r"
                display: flex;
                flex-flow: row;
                height: 100%;
                max-height: 100%;
                flex: 1;
                background-color: #A4A6A5;
                overflow: auto;
            ",

            // Engine list
            div {
                style: "
                    min-height: 100%;
                    max-height: 100%;
                    min-width: 35em;
                    padding: 0.5em;
                    padding-bottom: 8em;
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
                    class: "box",
                    style: r"
                        flow-direction: column;
                        height: 100%;
                        padding: 0.5em;
                        overflow-y: scroll;
                    ",

                    for engine in sorted_engines {
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