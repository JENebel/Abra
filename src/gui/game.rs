use super::*;

pub fn Game(cx: Scope, interactible: bool) -> Element {

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

            // Menu
            div {
                style: "
                    height: 100%;
                    min-width: 35em; 
                ",

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
                                    "Cadabra 1.0.2"
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
            div {
                style: "
                    flex: 1;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100%;
                    margin: 1em;
                ",
                id: "board_container",
                Board(cx)
            }
        }
    })
}