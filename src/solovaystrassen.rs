use super::thread_rng;
use super::{BigUint, RandBigInt};
use super::{Primality, ProbabilisticPrimalityTester};

pub struct SolovayStrassen;

fn jacobi_symbol(a: BigUint, n: BigUint) -> Option<i8> {
    // The LSB must be one.
    if !n.bit(0) {
        return None;
    }

    let mut a = a;
    let mut n = n;

    a %= &n;

    let mut t = 1;

    while a != 0u8.into() {
        if let Some(num) = a.trailing_zeros() {
            for _ in 0..num {
                a >>= 1;
                let r: BigUint = &n % 8u8;

                if r == 3u8.into() || r == 5u8.into() {
                    t = -t;
                }
            }
        }

        std::mem::swap(&mut a, &mut n);

        if &a % 4u8 == 3u8.into() && &n % 4u8 == 3u8.into() {
            t = -t;
        }

        a &= &n;
    }

    if n == 1u8.into() {
        return Some(t);
    }

    Some(0)
}

impl ProbabilisticPrimalityTester for SolovayStrassen {
    fn test(n: &BigUint, rounds: u32) -> Primality {
        let mut rng = thread_rng();

        let legendre_power = (n - 1u8) >> 1;

        for _ in 0..rounds {
            let a = rng.gen_biguint_range(&2u8.into(), n);
            let x = jacobi_symbol(a.clone(), n.clone()).unwrap();

            let x = match x.signum() {
                0 => return Primality::Composite,
                -1 => (n - (-x as u8)) % n,
                1 => (n + (x as u8)) % n,
                _ => unreachable!(),
            };

            if a.modpow(&legendre_power, n) != x {
                return Primality::Composite;
            }
        }

        Primality::ProbablyPrime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jacobi() {
        // Tests some known jacobi symbol values
        assert_eq!(
            jacobi_symbol(BigUint::from(1u8), BigUint::from(1u8)),
            Some(1)
        );
        assert_eq!(
            jacobi_symbol(BigUint::from(2u8), BigUint::from(3u8)),
            Some(-1)
        );
        assert_eq!(
            jacobi_symbol(BigUint::from(5u8), BigUint::from(51u8)),
            Some(1)
        );
        assert_eq!(
            jacobi_symbol(BigUint::from(21u8), BigUint::from(53u8)),
            Some(-1)
        );
        assert_eq!(
            jacobi_symbol(BigUint::from(6u8), BigUint::from(21u8)),
            Some(0)
        );
    }

    #[test]
    fn large_prime() {
        const LARGE_PRIMES: [&str; 1] = ["359334085968622831041960188598043661065388726959079837"];

        for prime in LARGE_PRIMES {
            let prime = BigUint::parse_bytes(prime.as_bytes(), 10).unwrap();
            assert_eq!(SolovayStrassen::test(&prime, 2), Primality::ProbablyPrime);
        }
    }

    #[test]
    fn small_primes() {
        const SMALL_PRIMES: [u32; 52] = [
            0x05, 0x07, 0x0B, 0x0D, 0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25, 0x29, 0x2B, 0x2F, 0x35,
            0x3B, 0x3D, 0x43, 0x47, 0x49, 0x4F, 0x53, 0x59, 0x61, 0x65, 0x67, 0x6B, 0x6D, 0x71,
            0x7F, 0x83, 0x89, 0x8B, 0x95, 0x97, 0x9D, 0xA3, 0xA7, 0xAD, 0xB3, 0xB5, 0xBF, 0xC1,
            0xC5, 0xC7, 0xD3, 0xDF, 0xE3, 0xE5, 0xE9, 0xEF, 0xF1, 0xFB,
        ];

        for prime in SMALL_PRIMES {
            assert_eq!(
                SolovayStrassen::test(&prime.into(), 2),
                Primality::ProbablyPrime
            );
        }
    }

    #[test]
    fn test_non_prime() {
        const SOME_NON_PRIMES: [&str; 2] = ["25", "235"];
        for non_prime in SOME_NON_PRIMES {
            let non_prime = BigUint::parse_bytes(non_prime.as_bytes(), 10).unwrap();
            assert_eq!(SolovayStrassen::test(&non_prime, 2), Primality::Composite);
        }
    }
}
