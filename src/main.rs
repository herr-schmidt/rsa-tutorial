use std::ops::Div;

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::Zero;

fn main() {
    let (prime, trials) = find_prime();
    println!("key = {}", prime);
    println!("trials = {}", trials);
}

fn find_prime() -> (BigUint, u32) {
    let mut rng = rand::thread_rng();
    let bit_size = 512;
    let mut candidate = rng.gen_biguint(bit_size);
    let mut trials = 0;
    while !miller_rabin(&candidate, &10) {
        candidate = rng.gen_biguint(bit_size);
        trials += 1;
    }
    return (candidate, trials);
}

fn miller_rabin(candidate: &BigUint, iterations: &u32) -> bool {
    let mut t = 0;

    let big_uint_one = 1.to_biguint().unwrap();
    let big_uint_two = 2.to_biguint().unwrap();

    while &t < iterations {
        let mut inconclusive = false;

        let mut n = candidate - &big_uint_one;
        let mut k = 0;
        let q: BigUint;

        loop {
            if n.modpow(&big_uint_one, &big_uint_two) == BigUint::zero() {
                k = k + 1;
                n = n.div(&big_uint_two);
            } else {
                q = n;
                break;
            }
        }
        if k == 0 {
            return false;
        }

        let mut rng = rand::thread_rng();
        let low = &big_uint_two;
        let high = &(candidate - &big_uint_two);
        let a = rng.gen_biguint_range(low, high);

        if a.modpow(&q, candidate) == big_uint_one {
            inconclusive = true;
        }

        for j in 0..k {
            let exponent = &big_uint_two.pow(j) * &q;
            if a.modpow(&exponent, candidate) == candidate - &big_uint_one {
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
