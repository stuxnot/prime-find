use super::BigUint;
use super::Primality;

fn gcd(u: &BigUint, v: &BigUint) -> BigUint {
    let swap = u == v;

    let mut g: BigUint = 1u32.into();
    let mut utmp: BigUint = if swap { v.clone() } else { u.clone() };
    let mut vtmp: BigUint = if swap { u.clone() } else { v.clone() };

    while &utmp & &1u32.into() == 0u32.into() && &vtmp & &1u32.into() == 0u32.into() {
        utmp >>= 1u32;
        vtmp >>= 1u32;
        g <<= 1u32;
    }

    while &utmp != &0u32.into() {
        while &utmp & &1u32.into() == 0u32.into() {
            utmp >>= 1u32;
        }
        while &vtmp & &1u32.into() == 0u32.into() {
            vtmp >>= 1u32;
        }

        if utmp >= vtmp {
            utmp -= &vtmp;
            utmp >>= 1u32;
        } else {
            vtmp -= &utmp;
            vtmp >>= 1u32;
        }
    }

    g * vtmp
}

pub fn prime_test_trial(n: &BigUint) -> Primality {
    const SMALL_PRIMES: [u32; 53] = [
        0x03, 0x05, 0x07, 0x0B, 0x0D, 0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25, 0x29, 0x2B, 0x2F, 0x35,
        0x3B, 0x3D, 0x43, 0x47, 0x49, 0x4F, 0x53, 0x59, 0x61, 0x65, 0x67, 0x6B, 0x6D, 0x71, 0x7F,
        0x83, 0x89, 0x8B, 0x95, 0x97, 0x9D, 0xA3, 0xA7, 0xAD, 0xB3, 0xB5, 0xBF, 0xC1, 0xC5, 0xC7,
        0xD3, 0xDF, 0xE3, 0xE5, 0xE9, 0xEF, 0xF1, 0xFB,
    ];

    for prime in SMALL_PRIMES {
        if *n == prime.into() {
            break;
        }

        let g = gcd(n, &prime.into());
        if g != 1u32.into() {
            return Primality::Composite;
        }
    }

    Primality::ProbablyPrime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&5u32.into(), &15u32.into()), 5u32.into());
        assert_eq!(gcd(&12u32.into(), &8u32.into()), 4u32.into());
    }

    #[test]
    fn large_prime() {
        const LARGE_PRIMES: [&str; 1] = ["359334085968622831041960188598043661065388726959079837"];

        for prime in LARGE_PRIMES {
            let prime = BigUint::parse_bytes(prime.as_bytes(), 10).unwrap();
            assert_eq!(prime_test_trial(&prime), Primality::ProbablyPrime);
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
            assert_eq!(prime_test_trial(&prime.into()), Primality::ProbablyPrime);
        }
    }

    #[test]
    fn test_non_prime() {
        const SOME_NON_PRIMES: [&str; 2] = ["25", "235"];
        for non_prime in SOME_NON_PRIMES {
            let non_prime = BigUint::parse_bytes(non_prime.as_bytes(), 10).unwrap();
            assert_eq!(prime_test_trial(&non_prime), Primality::Composite);
        }
    }
}
