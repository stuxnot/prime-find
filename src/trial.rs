use super::BigUint;
use super::Primality;

fn gcd(mut u: BigUint, mut v: BigUint) -> BigUint {
    if u == 0u32.into() {
        return v;
    }

    if v == 0u32.into() {
        return u;
    }

    // SAFETY:
    // Since u and v are not 0, we can unwrap safely.
    let u_divisor_exponent_on_two = u.trailing_zeros().unwrap();
    let v_divisor_exponent_on_two = v.trailing_zeros().unwrap();

    let shared_exponent_on_two =
        std::cmp::min(u_divisor_exponent_on_two, v_divisor_exponent_on_two);

    u >>= u_divisor_exponent_on_two;
    v >>= v_divisor_exponent_on_two;

    while u != v {
        if u < v {
            std::mem::swap(&mut u, &mut v);
        }

        u -= &v;
        // SAFETY:
        // We have ensured that u > v, thus u - v > 0.
        u >>= u.trailing_zeros().unwrap();
    }

    u << shared_exponent_on_two
}

/// Tests a number `n` for primality by calculating the gcd of products of small primes
/// and the number. For numbers smaller or equal to the largest number in the products,
/// this test fails, since if `n` is prime, it will be part of the product.
///
/// # Returns
/// Some(Primality) for n > 0xFB, None otherwise
pub fn prime_test_trial(n: &BigUint) -> Option<Primality> {
    const SMALL_PRIMES_PRODUCTS: [u64; 6] = [
        0x03 * 0x05
            * 0x07
            * 0x0B
            * 0x0D
            * 0x11
            * 0x13
            * 0x17
            * 0x1D
            * 0x1F
            * 0x25
            * 0x29
            * 0x2B
            * 0x2F
            * 0x35,
        0x3B * 0x3D * 0x43 * 0x47 * 0x49 * 0x4F * 0x53 * 0x59 * 0x61 * 0x65,
        0x67 * 0x6B * 0x6D * 0x71 * 0x7F * 0x83 * 0x89 * 0x8B * 0x95,
        0x97 * 0x9D * 0xA3 * 0xA7 * 0xAD * 0xB3 * 0xB5 * 0xBF,
        0xC1 * 0xC5 * 0xC7 * 0xD3 * 0xDF * 0xE3 * 0xE5 * 0xE9,
        0xEF * 0xF1 * 0xFB,
    ];

    if n <= &0xFBu32.into() {
        return None;
    }

    for product in SMALL_PRIMES_PRODUCTS {
        let g = gcd(n.clone(), product.into());
        if g != 1u32.into() {
            return Some(Primality::Composite);
        }
    }

    Some(Primality::ProbablyPrime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(5u32.into(), 15u32.into()), 5u32.into());
        assert_eq!(gcd(12u32.into(), 8u32.into()), 4u32.into());
    }

    #[test]
    fn large_prime() {
        const LARGE_PRIMES: [&str; 1] = ["359334085968622831041960188598043661065388726959079837"];

        for prime in LARGE_PRIMES {
            let prime = BigUint::parse_bytes(prime.as_bytes(), 10).unwrap();
            assert_eq!(prime_test_trial(&prime).unwrap(), Primality::ProbablyPrime);
        }
    }

    #[test]
    fn test_non_prime() {
        const SOME_NON_PRIMES: [&str; 2] = ["2500", "235182349"];
        for non_prime in SOME_NON_PRIMES {
            let non_prime = BigUint::parse_bytes(non_prime.as_bytes(), 10).unwrap();
            assert_eq!(prime_test_trial(&non_prime).unwrap(), Primality::Composite);
        }
    }
}
