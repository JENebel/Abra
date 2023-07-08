use super::*;

pub struct Tourney {
    //book: Option<Book>,
    max_concurrent_games: usize,
    engines: Vec<Engine>,
    planned: Vec<PlannedGame>,
    ongoing: Vec<Game>,
    finished: Vec<FinishedGame>,
}

impl Tourney {
    pub fn new () -> Self {
        Self {
            //book: None,
            max_concurrent_games: 1,
            engines: Vec::new(),
            planned: Vec::new(),
            ongoing: Vec::new(),
            finished: Vec::new(),
        }
    }

    pub fn clear_games(&mut self) {
        self.engines.clear();
        self.planned.clear();
        self.ongoing.clear();
        self.finished.clear();
    }

    pub fn add_engine(&mut self, engine: Engine) {
        self.engines.push(engine);
    }

    pub fn add_games(&mut self, games: Vec<PlannedGame>) {
        self.planned.extend(games);
    }
}