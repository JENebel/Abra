use std::collections::HashMap;

use super::*;

pub fn EngineInfo<'a>(cx: Scope<'a>, id: Option<u32>, engines: &'a UseSharedState<HashMap<u32, Engine>>) -> Element<'a> {
    //let engines = use_shared_state::<EngineMap>(cx).unwrap();
    let engine:Option<Engine> = match id {
        None => None,
        Some(id) => engines.read().get(&id).cloned(),
    };

    let input_field_style = r"
        display: flex;
        flex-flow: row;
        align-items: center;
        margin-top: 0.5em;
        justify-content: space-between;
        max-width: 30em;
    ";

    let input_box_style = r"
        margin-left: 0.5em;
        font-size: 1.5em;
        width: 80%;
        type: text;
        self-align: flex-end;
    ";

    let input_paragraph_style = r"
        margin: 0;
        font-size: 1.5em;
        self-align: flex-start;
    ";

    match engine {
        None => cx.render(rsx!{ div {
            style: "padding: 3em; font-weight: bold; font-size: 1.5",
            "No engine selected"
        }}),
        Some(engine) => cx.render(rsx!{
            // Container
            div {
                style: r"
                    display: flex;
                    flex-flow: column;
                    margin: 0.5em;
                    padding: 0.5em;
                    width: 50em;
                ",

                // Buttons
                div {
                    
                },

                p { 
                    style: r"
                        font-weight: bold;
                        font-size: 2em;
                        margin-top: 0;
                    ",

                    "Information"
                },
                p {
                    style: r"
                        margin-top: -0.5em;
                        font-size: 1.5em;
                    ",

                    "Name: {engine.name}"
                },
                p {
                    style: r"
                        margin-top: -0.5em;
                        margin-bottom: 0.2em;
                        font-size: 1.5em;
                    ",

                    "Author: {engine.author}"
                },

                // Edit alias
                div {
                    style: input_field_style,
                    p { style: input_paragraph_style, "Alias:" },
                    input {
                        style: input_box_style,
                        value: "{engine.alias}",
                        oninput: move |event| {
                            let mut engines = engines.write();
                            let engine = engines.get_mut(&engine.id).unwrap();
                            engine.alias = event.inner().value.to_string();
                            store_engine(engine).unwrap();
                        }
                    }
                },

                // Edit Elo
                div {
                    style: input_field_style,
                    p { style: input_paragraph_style, "Elo:" },
                    input {
                        style: input_box_style,
                        value: "{engine.elo}",
                        oninput: move |event| {
                            match event.inner().value.to_string().parse::<u32>() {
                                Ok(elo) => {
                                    let mut engines = engines.write();
                                    let engine = engines.get_mut(&engine.id).unwrap();
                                    engine.elo = elo;
                                    store_engine(engine).unwrap();
                                },
                                Err(_) => println!("Invalid Elo"),
                            }
                        }
                    }
                },

                // Edit path
                div {
                    style: r"
                        {input_field_style};
                        height: 2em;
                    ",
                    p { style: input_paragraph_style, "Path:" },
                    p {
                        style: r"
                            white-space: nowrap;
                            overflow: hidden;
                            text-overflow: ellipsis;
                            direction: rtl;
                            {input_box_style}
                            self-align: flex-end;
                            margin-left: 1em;
                            
                            width: 72%;
                        ",
                        "{engine.path.to_str().unwrap()}",
                    },
                    p {
                        class: "file",
                        style: r"
                            text-align: right;
                            font-size: 1.5em;
                        ",

                        onclick: move |_event| {
                            let mut engines = engines.write();
                            let path = match open_file_dialog() {
                                Err(_) => return,
                                Ok(p) => p,
                            };
                            let engine = engines.get_mut(&engine.id).unwrap();
                            engine.path = path;
                            store_engine(engine).unwrap();
                        },

                        "📂"
                    }
                },
            }
        }),
    }
}