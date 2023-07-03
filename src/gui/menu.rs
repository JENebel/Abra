use super::*;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx!{
        // use a for loop where the body itself is RSX
        div {
            style: "display: flex; flex-direction: row; padding-left: 0.5em; padding-top: 0.25em;",
            background: "lightgray",
            border_bottom: "solid 0.16em gray",

            MenuButton(cx, Page::Game),
            MenuButton(cx, Page::Tourney),
            MenuButton(cx, Page::Editor),
            MenuButton(cx, Page::Engines),
            MenuButton(cx, Page::Books),
            MenuButton(cx, Page::Settings),
            MenuButton(cx, Page::About),
        }
    })
}

pub fn MenuButton<'a>(cx: Scope<'a>, page: Page) -> Element<'a> {
    let active_page = use_shared_state::<Page>(cx).unwrap();
    let text = format!("{:?}", page);

    let active = *active_page.read() == page;

    let button = rsx!{
        button {
            class: "pure-button",
            style: "margin: 0.25em; margin-bottom: 0.5em; margin-top: 0.25em; height: 3em; visibility: visible; width: 8em; font-weight: bold;",
            onclick: move |_event| {
                if !active {
                    *active_page.write() = page;
                }
            },
            "{text}",
        }
    };

    let visibility = if active { "visible" } else { "hidden" };

    cx.render(
        rsx!{
            div {
                class: "box",
                style: r"
                    display: flex;
                    flex-direction: row;
                    margin: 0;
                    margin-top: 0.25em;
                    box-sizing: border-box;
                    border-bottom-left-radius: 0;
                    border-bottom-right-radius: 0;
                    margin-bottom: -0.16em;
                    border-width: 0.16em;
                    border-bottom: none;
                    background-color: #A4A6A5;
                    visibility: {visibility};
                ",
                button,
            }
        }
    )
}