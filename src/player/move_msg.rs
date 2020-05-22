use super::*;

#[derive(WriteTo, ReadFrom, Clone)]
pub struct MoveMsg {
    pub id: u8,
    pub coord: (u16, u16),
}

pub enum PlayerMoveError {
    PlayerNotFound(u8),
    InvalidMove(u8),
}

impl std::fmt::Display for PlayerMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PlayerNotFound(id) => {
                write!(f, "player {} not found", id)
            }
            Self::InvalidMove(id) => {
                write!(f, "invalid move from player {}", id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let not_found_err = PlayerMoveError::PlayerNotFound(3);
        assert_eq!(format!("{}", not_found_err), "player 3 not found");
        let invalid_move_err = PlayerMoveError::InvalidMove(4);
        assert_eq!(format!("{}", invalid_move_err), "invalid move from player 4");
    }
}

