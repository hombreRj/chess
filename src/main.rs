pub mod piece;

use piece::ChessPiece;

fn main() {
    let pawn_one: ChessPiece = ChessPiece {
        name: String::from("Pawn"),
        team: String::from("Red"),
        symbol: String::from("X"),
        move_anywhere: false,
        x_location: 2,
        y_location: 0,
        first_move: true,
    };

    println!("{:?}", pawn_one);

    let result: bool = pawn_one._can_move(2, 1);

    println!("Can {:?} move to x=2 y=1 {result}", pawn_one)
}
