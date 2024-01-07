use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;
use std::env;

use prime_find::{prime_test_trial, Fermat, MillerRabin, Primality, ProbabilisticPrimalityTester};

fn find_prime(bits: u64) -> BigUint {
    let mut rng = thread_rng();

    let mut prime;
    loop {
        prime = rng.gen_biguint(bits);
        prime.set_bit(0, true);

        if prime_test_trial(&prime) == Some(Primality::Composite) {
            continue;
        }

        if Fermat::test(&prime, 2) == Primality::ProbablyPrime {
            if MillerRabin::test(&prime, 3) == Primality::ProbablyPrime {
                break;
            }
        }
    }
    prime
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} [num-bits]", args[0]);
        return;
    }

    if let Ok(bits) = args[1].parse() {
        println!("Searching for prime with {bits} bits:");
        println!("Found:\n{}", find_prime(bits));
    } else {
        println!("Could not parse number of bits.");
    }
}
