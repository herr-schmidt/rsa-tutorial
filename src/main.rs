use std::ops::Div;

use num_bigint::{BigInt, RandBigInt, Sign::Plus, ToBigInt};
use num_traits::{Signed, Zero};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let mut generator = rand::rngs::StdRng::seed_from_u64(553246);
    let mut p = BigInt::zero();
    let mut q = BigInt::zero();
    let mut n = BigInt::zero();
    let mut phi_n = BigInt::zero();

    let mut d = BigInt::zero();

    let e = 7.to_bigint().unwrap();
    loop {
        let mut primes: Vec<BigInt> = find_primes(2, &mut generator);
        p = primes.pop().unwrap();
        q = primes.pop().unwrap();
        n = &p * &q;
        phi_n = (&p - 1) * (&q - 1);

        // (x, y, gcd)
        let mut extended_euclid_result = (BigInt::zero(), BigInt::zero(), BigInt::zero());
        extended_euclid_result = extended_euclid(&phi_n, &e);

        if extended_euclid_result.2 == 1.to_bigint().unwrap() {
            d = extended_euclid_result.1.clone();
            if d < BigInt::zero() {
                d = (&phi_n + &extended_euclid_result.1).modpow(&1.to_bigint().unwrap(), &phi_n);
            }
            break;
        }
    }

    println!("p: {}", &p);
    println!("q: {}", &q);
    println!("n = pq: {}", &p * &q);

    // public key: (e, n)
    // private key: (d, n)

    let message = String::from("Super secret message!!!");
    let message_as_bytes = message.as_bytes();
    let message_as_integer = BigInt::from_bytes_le(Plus, message_as_bytes);

    println!("message: {}", message);
    println!("message as integer: {}", message_as_integer);

    let encoded_message_as_integer = message_as_integer.modpow(&e, &n);
    let encoded_message_as_bytes = encoded_message_as_integer.to_bytes_le().1.to_owned();
    let encoded_message = String::from_utf8_lossy(&encoded_message_as_bytes);

    println!("encoded message: {}", &encoded_message);
    println!("encoded message as integer: {}", &encoded_message_as_integer);

    let decoded_message_as_integer = encoded_message_as_integer.modpow(&d, &n);
    let decoded_message_as_bytes = decoded_message_as_integer.to_bytes_le().1.to_owned();
    let decoded_message = String::from_utf8_lossy(&decoded_message_as_bytes);

    println!("encoded message: {}", &decoded_message);
    println!("decoded message as integer: {}", decoded_message_as_integer);
}

fn find_primes(to_find: u32, generator: &mut StdRng) -> Vec<BigInt> {
    let mut primes = Vec::new();
    for _ in 0..to_find {
        primes.push(find_prime(generator));
    }
    primes
}

fn find_prime(generator: &mut StdRng) -> BigInt {
    let mut candidate = BigInt::zero();
    loop {
        candidate = generator.gen_bigint(256).abs();
        let is_prime = miller_rabin(&candidate, &10);
        if is_prime {
            break;
        }
    }
    candidate
}

fn miller_rabin(candidate: &BigInt, iterations: &u32) -> bool {
    let mut t = 0;

    let big_int_one = 1.to_bigint().unwrap();
    let big_int_two = 2.to_bigint().unwrap();

    while &t < iterations {
        let mut inconclusive = false;

        let mut n = candidate - &big_int_one;
        let mut k = 0;
        let q: BigInt;

        loop {
            if n.modpow(&big_int_one, &big_int_two) == BigInt::zero() {
                k = k + 1;
                n = n.div(&big_int_two);
            } else {
                q = n;
                break;
            }
        }

        let mut rng = rand::thread_rng();
        let low = big_int_two.clone();
        let high = candidate - &big_int_two;
        let a = rng.gen_bigint_range(&low, &high);

        if a.modpow(&q, candidate) == big_int_one {
            inconclusive = true;
        }

        for j in 0..k {
            let exponent = &big_int_two.pow(j) * &q;
            if a.modpow(&exponent, candidate) == candidate - &big_int_one {
                inconclusive = true;
            }
        }
        if !inconclusive {
            return false; // composite
        }
        t += 1;
    }
    true // always inconclusive
}

fn extended_euclid(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_x = 1.to_bigint().unwrap();
    let mut x = BigInt::zero();
    let mut old_y = BigInt::zero();
    let mut y = 1.to_bigint().unwrap();
    while r != BigInt::zero() {
        let q = old_r.clone() / r.clone();
        (old_r, r) = (r.clone(), old_r - &q * r);
        (old_x, x) = (x.clone(), old_x - &q * x);
        (old_y, y) = (y.clone(), old_y - &q * y);
    }
    return (old_x, old_y, old_r);
}
