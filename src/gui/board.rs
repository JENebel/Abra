use shakmaty::{variant::VariantPosition, Position, Square, File, Rank, Role, Color};

use super::*;

pub fn Board(cx: Scope) -> Element {
    let game = use_shared_state::<Option::<Game>>(cx).unwrap();
    /*let game = Game::new_standard_game(Player::Human, Player::Human);

    let position = match game.current_position {
        VariantPosition::Chess(chess) => chess,
        _ => panic!("Not a chess game"),
    };*/

    cx.render(rsx!{
        // Container
        div {
            style: r"
                display: grid;
                grid-template-columns: repeat(8, 1fr);
                grid-template-rows: repeat(8, 1fr);
                aspect-ratio: 1 / 1;
                border: 3px solid gray;
                font-weight: bold;
                max-height: calc(100% - 0.125em);
            ",

            // Render chess board cells
            for row in (0..8).rev() {
                for col in 0..8 {
                    render! {
                        div {
                            style: r"
                                justify-content: center;
                                display: flex;
                                align-items: center;
                                aspect-ratio: 1 / 1;
                                font-size: 2em;
                            ",
                            background_color: if (row + col) % 2 == 0 { "Beige" } else { "#7d4d15" },

                            // Render chess pieces
                            img {
                                style: "width: 100%; height: 100%; image-rendering: pixelated;",
                                src: match game.read().as_ref().unwrap().current_position.board().piece_at(Square::from_coords(File::new(col), Rank::new(row))) {
                                    Some(piece) => match (piece.color, piece.role) {
                                        (Color::White, Role::King) => "src/gui/ChessPieces/WhiteKing.png",
                                        (Color::White, Role::Queen) => "src/gui/ChessPieces/WhiteQueen.png",
                                        (Color::White, Role::Rook) => "src/gui/ChessPieces/WhiteRook.png",
                                        (Color::White, Role::Bishop) => "src/gui/ChessPieces/WhiteBishop.png",
                                        (Color::White, Role::Knight) => "src/gui/ChessPieces/WhiteKnight.png",
                                        (Color::White, Role::Pawn) => "src/gui/ChessPieces/WhitePawn.png",
                                        (Color::Black, Role::King) => "src/gui/ChessPieces/BlackKing.png",
                                        (Color::Black, Role::Queen) => "src/gui/ChessPieces/BlackQueen.png",
                                        (Color::Black, Role::Rook) => "src/gui/ChessPieces/BlackRook.png",
                                        (Color::Black, Role::Bishop) => "src/gui/ChessPieces/BlackBishop.png",
                                        (Color::Black, Role::Knight) => "src/gui/ChessPieces/BlackKnight.png",
                                        (Color::Black, Role::Pawn) => "src/gui/ChessPieces/BlackPawn.png",
                                    },
                                    None => "",
                                },
                            }
                            /*if let Some(piece) = position.board().piece_at(Square::from_coords(File::new(col), Rank::new(row))) {
                                match (piece.color, piece.role) {
                                    (Color::White, Role::King) => "♔".to_string(),
                                    (Color::White, Role::Queen) => "♕".to_string(),
                                    (Color::White, Role::Rook) => "♖".to_string(),
                                    (Color::White, Role::Bishop) => "♗".to_string(),
                                    (Color::White, Role::Knight) => "♘".to_string(),
                                    (Color::White, Role::Pawn) => "♙".to_string(),
                                    (Color::Black, Role::King) => "♚".to_string(),
                                    (Color::Black, Role::Queen) => "♛".to_string(),
                                    (Color::Black, Role::Rook) => "♜".to_string(),
                                    (Color::Black, Role::Bishop) => "♝".to_string(),
                                    (Color::Black, Role::Knight) => "♞".to_string(),
                                    (Color::Black, Role::Pawn) => "♟".to_string(),
                                }.to_string()
                            } else {
                                "".to_string()
                            }*/
                        }
                    }
                }
            }
        }
    })
}