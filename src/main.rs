mod board;
mod hexagon;
mod line;
mod position;
mod ring;
mod segment;

use board::{Board, Puzzle};

fn main() {
    let puzzle = Puzzle::new(Board::random(3).unwrap());
    println!("{puzzle:?}");
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4)
    // }
}
