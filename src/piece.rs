struct Piece {
    name: String,
    team: String,
    symbol: String,
    move_anywhere: bool,
    x_location: u8,
    y_location: u8,
}

impl piece {
    fn canMove(&self, x: i8, y: i8) -> bool {
        if self.name == "King" {
            true
        } else if self.name == "Pawn" {
        }
    }
}
