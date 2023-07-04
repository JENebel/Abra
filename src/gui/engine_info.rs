use std::collections::HashMap;

use super::*;

pub fn EngineInfo<'a>(cx: Scope<'a>, id: Option<u32>, engines: &'a UseSharedState<HashMap<u32, Engine>>) -> Element<'a> {
    //let engines = use_shared_state::<EngineMap>(cx).unwrap();
    let engine: Option<Engine> = match id {
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

    cx.render(rsx!{
        div {
            class: "box",
            style: r"
                flex: 1;
                min-height: 100%;
                max-height: 100%;
                overflow: hidden;
                padding-left: 1em;
                padding-right: 1em;
            ",

            match engine {
                None => render!{ div {
                    p {
                        style: "font-size: 1.5em; font-weight: bold;",
                        "No engine selected."
                    }
                }},
                Some(engine) => render!{
                    // Container
                    ul {
                        style: r"
                            display: flex;
                            flex-flow: column;
                            max-height: 100%;
                            padding-left: 1em;
                            width: 35em;
                            margin-top: 0.5em;
                        ",

                        // Buttons
                        div {
                            style: r"
                                display: flex;
                                flex-flow: row;
                                justify-content: space-between;
                                margin-top: -0.5em;
                            ",

                            p {
                                style: "font-size: 1.8em; font-weight: bold; self-align: flex-start;",
                                "Information"
                            }
    
                            div {
                                style: r"
                                    margin-top: 1.5em;
                                    display: flex;
                                    self-align: flex-end;
                                    margin-left: auto;
                                    height: 2.2em;
                                ",

                                // Reload
                                button {
                                    class: "pure-button",
                                    onclick: move |_event| {
                                        let mut engines = engines.write();
                                        let engine = engines.get_mut(&engine.id).unwrap();
                                        if engine.reload().is_err() {
                                            println!("Failed to reload engine!");
                                        }
                                        store_engine(engine).unwrap();
                                    },
                                    "Reload"
                                }
                                
                                // Uninstall
                                button {
                                    class: "pure-button",
                                    style: r"margin-left: 1em;",
                                    onclick: move |_event| {
                                        let mut engines = engines.write();
                                        engines.remove(&engine.id);
                                        remove_engine(engine.id).unwrap();
                                    },
                                    "Uninstall"
                                },
                            }
                        }

                        p {
                            style: r"
                                margin-top: -0.5em;
                                font-size: 1.5em;
                                overflow: visible;
                                white-space: nowrap;
                            ",
        
                            "Name: {engine.name}"
                        },
                        p {
                            style: r"
                                margin-top: -0.5em;
                                margin-bottom: 0.2em;
                                font-size: 1.5em;
                                overflow: visible;
                                white-space: nowrap;
                            ",
        
                            "Author(s): {engine.author}"
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
                                r#type: "number",
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
                                    let path = match open_file_dialog_with_path(engine.path.clone()) {
                                        Err(_) => return,
                                        Ok(p) => p,
                                    };
                                    // TODO: Check if file is valid
                                    let engine = engines.get_mut(&engine.id).unwrap();
                                    engine.path = path;
                                    if engine.reload().is_err() {
                                        println!("Failed to reload engine!");
                                        return;
                                    }
                                    store_engine(engine).unwrap();
                                },
        
                                "📂"
                            },
                        },
                        // Book selection
                        div {
                            style: input_field_style,
                            p { style: input_paragraph_style, "Book:" },
                            select {
                                style: "{input_box_style} width: 81.6%;",
                                value: "engine.book",
                                onchange: move |_event| {
                                    let mut engines = engines.write();
                                    let _engine = engines.get_mut(&engine.id).unwrap();
                                    // TODO: Implement
                                    //engine.book = event.inner().value.to_string();
                                    //store_engine(engine).unwrap();
                                    println!("Not implemented!")
                                },
                                option { value: "", "None" },
                                option { value: "book.bin", "book.bin" },
                                option { value: "book_small.bin", "book_small.bin" },
                            }
                        },

                        div {
                            style: r"
                                display: flex;
                                flex-flow: row;
                                justify-content: space-between;
                                margin-top: 1;
                                margin-bottom: -1em;
                            ",

                            p {
                                style: "font-size: 1.8em; font-weight: bold; margin-bottom: 0.5em;",
                                "Options"
                            }
    
                            // Restore defaults
                            button {
                                class: "pure-button",
                                style: r"
                                    width: 10em;
                                    margin-top: 1.5em;
                                    display: flex;
                                    self-align: flex-end;
                                    margin-left: auto;
                                    height: 2.2em;
                                ",
    
                                onclick: move |_event| {
                                    let mut engines = engines.write();
                                    let engine = engines.get_mut(&engine.id).unwrap();
                                    engine.restore_defaults();
                                    store_engine(engine).unwrap();
                                },
                                "Restore Defaults"
                            }
                        }

                        // Options
                        div {
                            class: "box",
                            style: r"
                                display: flex;
                                flex-flow: column;
                                flex: 1;
                                overflow-y: auto;
                                margin-bottom: 2em;
                                margin-left: 0em;
                                min-height: 100%;
                                max-height: 100%;
                                width: 35em;
                                padding-bottom: 0.5em;
                            ",

                            div {
                                style: r"
                                    display: flex;
                                    flex-flow: row;
                                    max-width: 30em;
                                    min-width: 30em;
                                    content-justify: space-between;
                                    min-height: 3em;
                                    max-height: 3em;
                                    margin: 0.5em;
                                    margin-top: 0em;
                                    border-bottom: 1px dotted black;
                                ",
                    
                                p {
                                    style: r"
                                        margin-right: 1em;
                                        width: 100%
                                        text-align: center;
                                    ",
                    
                                    "Option name"
                                }

                                p {
                                    style: r"
                                        min-width: 5em;
                                        max-width: 5em;
                                        self-align: flex-end;
                                        text-align: center;
                                        margin-left: auto;
                                    ",
            
                                    "Default"
                                }
                            }

                            for opt in engine.options {
                                if !opt.is_uci_option && !matches!(opt.inner, InnerUCIOption::Button) {
                                    Option(cx, engine.id, engines, opt)
                                }
                            }
                        }
                    }
                },
            }
        }
    })
}

pub fn Option<'a>(cx: Scope<'a>, id: u32, engines: &'a UseSharedState<HashMap<u32, Engine>>, option: UCIOption) -> Element<'a> {
    cx.render(rsx! {
        div {
            style: r"
                display: flex;
                flex-flow: row;
                max-width: 30em;
                min-width: 30em;
                content-justify: space-between;
                min-height: 3em;
                max-height: 3em;
                margin: 0.5em;
                margin-top: 0em;
                margin-bottom: -0.5em;
            ",

            p {
                style: r"
                    margin-right: 1em;
                    min-width: 10em;
                    max-width: 10em;
                ",

                "{option.name}"
            }
            
            match option.inner {
                InnerUCIOption::Check(value, default) => render! { 
                    input {
                        r#type: "checkbox",
                        name: "Brian",
                        style: "margin: 0.5em; self-align: flex-center; height: 1em; width: 1em; margin-top: 1em;",
                        checked: value,
                        onchange: move |event| {
                            let mut engines = engines.write();
                            let engine = engines.get_mut(&id).unwrap();
                            if engine.set_option(&option.name, &event.inner().value).is_err() {
                                println!("Failed to set option!");
                                return;
                            }
                            store_engine(engine).unwrap();
                        },
                    }

                    p {
                        style: r"
                            min-width: 5em;
                            max-width: 5em;
                            self-align: flex-end;
                            margin-left: auto;
                            text-align: center;
                        ",

                        if default { "Yes" } else { "No" }
                    }
                },
                InnerUCIOption::Spin(value, default, min, max) => render! { 
                    input {
                        r#type: "number",
                        min: "{min}",
                        max: "{max}",
                        value: "{value}",
                        step: 1,
                        pattern: "[0-9]+",
                        style: "width: 100%; margin: 0.5em; self-align: flex-end; text-align: center;",
                        onchange: move |event| {
                            let mut engines = engines.write();
                            let engine = engines.get_mut(&id).unwrap();
                            if engine.set_option(&option.name, &event.inner().value).is_err() {
                                println!("Failed to set option!");
                                return;
                            }
                            store_engine(engine).unwrap();
                        },

                        "Spin"
                    }

                    p {
                        style: r"
                            min-width: 7em;
                            max-width: 7em;
                            self-align: flex-end;
                            text-align: center;
                        ",

                        "[{min}, {max}]"
                    }

                    p {
                        style: r"
                            min-width: 5em;
                            max-width: 5em;
                            self-align: flex-end;
                            text-align: center;
                            margin-left: auto;
                        ",

                        "{default}"
                    }
                },
                InnerUCIOption::Combo(value, default, vars) => render! { 
                    select {
                        style: "width: 100%; margin: 0.5em; self-align: flex-end; text-align: center;",
                        value: "{value}",
                        onchange: move |event| {
                            let mut engines = engines.write();
                            let engine = engines.get_mut(&id).unwrap();
                            if engine.set_option(&option.name, &event.inner().value).is_err() {
                                println!("Failed to set option!");
                                return;
                            }
                            store_engine(engine).unwrap();
                        },

                        for var in vars {
                            option { value: "{var}", selected: "{value == var}", "{var}" }
                        }
                    },

                    p {
                        style: r"
                            min-width: 5em;
                            max-width: 5em;
                            self-align: flex-end;
                            text-align: center;
                            margin-left: auto;
                        ",

                        "{default}"
                    }
                },
                InnerUCIOption::Button => todo!() /* render! {
                    // TODO: Implement
                    button {
                        class: "pure-button",
                        style: "self-align: flex-center; size: 100%; margin: 0.5em;",
                        "{option.name}"
                    }

                    p {
                        style: r"
                            min-width: 5em;
                            max-width: 5em;
                            self-align: flex-end;
                            text-align: center;
                        ",

                        "-"
                    }
                }*/,
                InnerUCIOption::String(value, default) => render! { 
                    input {
                        r#type: "text",
                        style: "width: 100%; margin: 0.5em; self-align: flex-end; text-align: center;",
                        value: "{value}",
                        onchange: move |event| {
                            let mut engines = engines.write();
                            let engine = engines.get_mut(&id).unwrap();
                            if engine.set_option(&option.name, &event.inner().value).is_err() {
                                println!("Failed to set option!");
                                return;
                            }
                            store_engine(engine).unwrap();
                        },
                    }

                    p {
                        style: r"
                            min-width: 5em;
                            max-width: 5em;
                            self-align: flex-end;
                            text-align: center;
                            margin-left: auto;
                            overflow-right: visible;
                            white-space: nowrap;
                        ",

                        "\"{default}\""
                    }
                },
            }
        }
    })
}