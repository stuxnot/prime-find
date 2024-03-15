use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;
use std::env;

use prime_find::{prime_test_trial, MillerRabin, Primality, ProbabilisticPrimalityTester};

fn find_prime(bits: u64) -> BigUint {
    let mut rng = OsRng::default();

    loop {
        let mut prime = rng.gen_biguint(bits);
        prime.set_bit(0, true);

        if prime_test_trial(&prime) == Some(Primality::Composite) {
            continue;
        }

        // According to "Average Case Error Estimates For The String Porbable Prime Test"
        // by Damgard, Landrock, and Pomerance, 10 rounds should give enough certainty
        // for numbers with more than 400 bits (see table 1). The other numbers are currently
        // not exactly calculated, but since the test is extremely fast for small numbers
        // we can just use a large number of rounds.
        let rounds = match bits {
            0..=100 => 80,
            101..=400 => 40,
            _ => 10,
        };

        if MillerRabin::test(&prime, rounds) == Primality::ProbablyPrime {
            return prime;
        }
    }
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
