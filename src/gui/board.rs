use super::*;

pub fn Board(cx: Scope) -> Element {
    cx.render(rsx!{
        // Container
        div {
            style: r"
                display: grid;
                grid-template-columns: repeat(8, 1fr);
                grid-template-rows: repeat(8, 1fr);
                aspect-ratio: 1 / 1;
                border: 3px solid gray;
                font-size: 5em;
                font-weight: bold;
                max-height: 100%;
            ",

            // Render chess board cells
            for row in 0..8 {
                for col in 0..8 {
                    render! {
                        div {
                            style: r"
                                justify-content: center;
                                display: flex;
                                align-items: center;
                            ",
                            background_color: if (row + col) % 2 == 0 { "Beige" } else { "#7d4d15" },

                            
                        }
                    }
                }
            }
        }
    })
}