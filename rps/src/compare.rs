pub enum RPSCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutsPaper
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}
