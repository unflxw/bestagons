mod board;
mod hexagon;
mod line;
mod position;
mod ring;
mod segment;

use board::generate_good;
use rand::{rngs::StdRng, SeedableRng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    // let mut rng = StdRng::seed_from_u64(22);
    generate_good(&mut rng, 2);
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4)
    // }
}
