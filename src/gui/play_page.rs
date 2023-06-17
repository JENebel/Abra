use super::*;

pub fn PlayPage(cx: Scope) -> Element {
    cx.render(rsx!{
        button { onclick: move |_event| {
            println!("Clicked div");
            //active_page.write() = page;
        },

        "Play"
    }})
}