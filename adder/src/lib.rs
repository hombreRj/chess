// #[cfg(test)]
// mod tests {
//     mod piece {
//         pub use super::ChessPiece;
//         #[test]
//         fn check_pawn_movement() {
//             let pawn_one: ChessPiece = ChessPiece {
//                 name: String::from("Pawn"),
//                 team: String::from("Red"),
//                 symbol: String::from("X"),
//                 move_anywhere: false,
//                 x_location: 2,
//                 y_location: 0,
//                 first_move: true,
//             };

//             let result: bool = pawn_one.can_move(2, 1);
//             assert_eq!(result, true);
//         }
//     }
// }
