mod grid;
mod puzzle;

use puzzle::generator::generate_good;
use rand::{rngs::StdRng, thread_rng, SeedableRng};

fn main() {
    let mut rng = thread_rng();
    // let mut rng = StdRng::seed_from_u64(22);
    generate_good(&mut rng, 3);
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4)
    // }
}
