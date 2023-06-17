use super::*;

pub fn PlayPage(cx: Scope) -> Element {
    cx.render(rsx!{
        Game(cx, true)
    })
}