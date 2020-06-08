
pub enum Action {
    Move(Direction)
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up, Down, Left, Right
}