use super::*;

pub fn Menu(cx: Scope) -> Element {
    cx.render(rsx!{
        // use a for loop where the body itself is RSX
        nav {
            div {
                button { "Play" },
                button { "Tourney" },
            }
        }
    })
}