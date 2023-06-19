use super::*;

pub fn Board(cx: Scope) -> Element {
    cx.render(rsx!{
        // Container
        div {
            style: r"
                display: grid;
                grid-template-columns: repeat(8, 1fr);
                grid-template-rows: repeat(8, 1fr);
                margin: auto;
                flex: 1;
                aspect-ratio: 1 / 1;
                border: 3px solid gray;
            ",

            // Render chess board cells
            for row in 0..8 {
                for col in 0..8 {
                    render! {
                        div {
                            background_color: if (row + col) % 2 == 0 { "Beige" } else { "SaddleBrown" },
                        }
                    }
                }
            }
        }
    })
}