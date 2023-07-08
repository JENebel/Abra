use std::sync::{Arc, Mutex};

use shakmaty::*;
use shakmaty::variant::*;
use super::*;

pub enum Player {
    Human,
    Engine(Engine),
}

#[derive(Clone)]
pub struct Game {
    start_position: VariantPosition,
    time_control: TimeControl,
    white_time: u64,
    black_time: u64,
    pub current_position: VariantPosition,
    white: Option<Arc<Mutex<EngineWrapper>>>,
    black: Option<Arc<Mutex<EngineWrapper>>>,
    move_history: Arc<Mutex<Vec<Move>>>,
}

impl Game {
    pub fn new(white: Player, black: Player, start_position: VariantPosition, time_control: TimeControl) -> Self {
        let white = match white {
            Player::Human => None,
            Player::Engine(engine) => {
                let wrapper = EngineWrapper::launch(engine).unwrap();
                wrapper.send("uci");
                Some(Arc::new(Mutex::new(wrapper)))
            },
        };
        
        let black = match black {
            Player::Human => None,
            Player::Engine(engine) => {
                let wrapper = EngineWrapper::launch(engine).unwrap();
                wrapper.send("uci");
                Some(Arc::new(Mutex::new(wrapper)))
            },
        };

        Self {
            start_position: start_position.clone(),
            current_position: start_position,
            white,
            black,
            move_history: Arc::new(Mutex::new(Vec::new())),
            time_control: time_control,
            white_time: time_control.initial_time,
            black_time: time_control.initial_time,
        }
    }

    pub fn new_standard_game(white: Player, black: Player, time_control: TimeControl) -> Self {
        Self::new(white, black, VariantPosition::Chess(Chess::default()), time_control)
    }
}

pub enum GameResult {
    Win(Color),
    Draw(DrawType),
}

pub enum DrawType {
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
    ThreefoldRepetition,
    Agreement,
}

pub struct PlannedGame {
    id: u32,
    white_id: Engine,
    black_id: Engine,
    start_position: VariantPosition,
}

pub struct FinishedGame {
    white: Engine,
    black: Engine,
    start_position: VariantPosition,
    result: GameResult,
}