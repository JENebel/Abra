use super::*;

pub fn Engines(cx: Scope) -> Element {
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

                        onclick: move |_event| { },
                        "Install new"
                    },
                },

                // The list
                div {
                    style: r"
                        display: flex;
                        flex-flow: row;
                        height: 100%;
                        flex: 1;
                        background-color: #A4A6A5;
                    ",
                }
            },
            div {
                class: "box",
            }
        }
    })
}