use super::*;

pub fn GamePage(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Option::<Game>::None);
    let game = use_shared_state::<Option::<Game>>(cx).unwrap();

    cx.render(rsx!{
        // Container
        div {
            class: "page",

            // Menu
            div {
                class: "side-panel box",
                div {
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

                div {
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
            },
            div {
                style: "
                    margin: 1em;
                    width: 100%;
                ",
                id: "board_container",
                Board(cx)
            }
        }
    })
}