use piece::piece;

impl piece {
    fn canMove(&self, x: i8, y: i8) -> bool {
        if self.name == "King" {
            true
        } else if self.name == "Pawn" {
        }
    }
}
