use super::*;

pub fn start_gui() {
    dioxus_desktop::launch(App)
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!{
        Menu(cx),
        // use a for loop where the body itself is RSX
        div {
            button {
                "Play"
            }
        }
    })
}