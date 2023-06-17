use super::*;

pub fn Game(cx: Scope, interactible: bool) -> Element {
    let layout_box = r"
        display: flex;
        margin: 0;
        padding: 0;
    ";

    let panel_style = r"
        display: inline-block;
        height: 100%;
        margin: 0.5em;
    ";

    cx.render(rsx!{
        div {
            style: "{layout_box} ",

            div {
                style: "{panel_style} width: 60%;",
                padding: "1.6em",
            },
            div {
                style: "{panel_style} width: 40%;",
                padding: "1.6em",

                div {
                    // If interactible show the play/pause, cancel, adjudicate and new buttons on a row in a rounded box
                    if interactible {
                        render! {
                            div {
                                class: "box",
                                style: "display: flex; flex-direction: row;",

                                button {
                                    class: "pure-button",
                                    margin: "0.5em",

                                    onclick: move |_event| { },
                                    "Start"
                                },
                                button {
                                    class: "pure-button",
                                    margin: "0.5em",

                                    onclick: move |_event| { },
                                    "Cancel"
                                },
                                button {
                                    class: "pure-button",
                                    margin: "0.5em",

                                    onclick: move |_event| { },
                                    "Adjudicate"
                                },
                                button {
                                    class: "pure-button",
                                    style: "margin: 0.5em; margin-left: auto;",

                                    onclick: move |_event| { },
                                    "New"
                                },
                            }
                        }
                    }

                    render! {
                        div {
                            class: "box",
                            style: "display: flex; flex-direction: row;",

                            div {
                                class: "box white",
                                margin: "0.5em",
                                width: "50%",

                                p {
                                    style: "margin: 0.5em; text-align: center; font-weight: bold; font-size: 1.2em; overflow: hidden; white-space: nowrap;",
                                    "Cadabra"
                                }

                                p {
                                    style: "margin: 0.5em; margin-top: -0.5em; text-align: center;; font-size: 0.8em; overflow: hidden; white-space: nowrap;",
                                    "by Joachim Enggaard Nebel"
                                }

                                p {
                                    style: "margin: 0.1em; text-align: center; font-weight: bold; font-size: 2.6em; overflow: hidden; white-space: nowrap;",
                                    "1:12"
                                }

                                p {
                                    style: "margin: 0.1em; text-align: center; font-size: 1.4em; overflow: hidden; white-space: nowrap;",
                                    "♞♟︎♟︎"
                                }
                            },
                            div {
                                class: "box black",
                                margin: "0.5em",
                                width: "50%",

                                p {
                                    style: "margin: 0.5em; text-align: center; font-weight: bold; font-size: 1.2em; overflow: hidden; white-space: nowrap;",
                                    "Stockfish 15.1"
                                }

                                p {
                                    style: "margin: 0.5em; margin-top: -0.5em; text-align: center; font-size: 0.8em; overflow: hidden; white-space: nowrap;",
                                    "by the Stockfish team"
                                }

                                p {
                                    style: "margin: 0.1em; text-align: center; font-weight: bold; font-size: 2.6em; overflow: hidden; white-space: nowrap;",
                                    "12:23"
                                }

                                p {
                                    style: "margin: 0.1em; text-align: center; font-size: 1.4em; overflow: hidden; white-space: nowrap;",
                                    "♕"
                                }
                            },
                        }
                    }
                }
            },
        }
    })
}