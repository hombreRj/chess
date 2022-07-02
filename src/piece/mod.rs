#[derive(Debug)]
pub struct ChessPiece {
    pub name: String,
    pub team: String,
    pub symbol: String,
    pub move_anywhere: bool,
    pub x_location: u8,
    pub y_location: u8,
    pub first_move: bool,
}

impl ChessPiece {
    pub fn _can_move(&self, x: u8, y: u8) -> bool {
        if self.name == "King" {
            true
        } else if self.name == "Pawn" {
            if self.x_location == x && self.y_location + 1 == y {
                return true;
            } else if self.x_location == x && self.y_location - 1 == y {
                return true;
            } else {
                if self.first_move {
                    if self.x_location == x && self.y_location == y + 2 {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            false
        } else {
            false
        }
    }
}
