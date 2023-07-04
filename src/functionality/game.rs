use shakmaty::*;
use shakmaty::variant::*;
use super::*;

pub enum Player {
    Human,
    Engine(Engine),
}
pub struct Game {
    start_position: VariantPosition,
    current_position: VariantPosition,
    white: Option<EngineWrapper>,
    black: Option<EngineWrapper>,
    move_history: Vec<Move>,
}

impl Game {
    pub fn new(white: Player, black: Player, start_position: VariantPosition) -> Self {
        let white = match white {
            Player::Human => None,
            Player::Engine(engine) => {
                let wrapper = EngineWrapper::launch(engine).unwrap();
                wrapper.send("uci");
                Some(wrapper)
            },
        };
        
        let black = match black {
            Player::Human => None,
            Player::Engine(engine) => {
                let wrapper = EngineWrapper::launch(engine).unwrap();
                wrapper.send("uci");
                Some(wrapper)
            },
        };

        Self {
            start_position: start_position.clone(),
            current_position: start_position,
            white, black,
            move_history: Vec::new(),
        }
    }

    pub fn new_standard_game(white: Player, black: Player) -> Self {
        Self::new(white, black, VariantPosition::Chess(Chess::default()))
    }
}
