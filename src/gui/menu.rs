use super::*;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx!{
        // use a for loop where the body itself is RSX
        div {
            class: "pure-menu pure-menu-horizontal",
            background: "#9c9c9c",
            MenuButton(cx, Page::Play),
            MenuButton(cx, Page::Tourney),
            MenuButton(cx, Page::Engines),
            MenuButton(cx, Page::Editor),
            MenuButton(cx, Page::Settings),
            MenuButton(cx, Page::Info),
        }
    })
}

pub fn MenuButton<'a>(cx: Scope<'a>, page: Page) -> Element<'a> {
    let active_page = use_shared_state::<Page>(cx).unwrap();
    let text = format!("{:?}", page);
    println!("Rendering menu button: {:?}", page);
    cx.render(rsx!{
        button {
            class: "pure-button",
            margin: "0.5em",
            onclick: move |_event| {
                println!("Clicked menu button: {:?}", page);
                //active_page.write() = page;
            },
            "{text}",
        }
    })
}