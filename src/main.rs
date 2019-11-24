extern crate collatz;
use collatz::Sequence;

use rand::Rng;

fn main() {
    let rng = &mut rand::thread_rng();
    for l in 0..=126 {
        for _ in 0..1000 {
            let mut next: u128 = rng.gen();
            if l != 128 {
                next = next % (1 << l);
            }

            if !Sequence::new(next, l).is_valid() {
                println!("NO: {:01$b}", next, l as usize);
            } else {
                //println!("OK: {:01$b}", next, l as usize);
            }
        }
    }
}
