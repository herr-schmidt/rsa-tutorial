use std::ops::Div;

use num_bigint::{BigUint, BigInt, RandBigInt, ToBigUint, ToBigInt};
use num_traits::Zero;

use std::sync::{Arc, Mutex};
use std::thread;

use rand::SeedableRng;

fn main() {
    let r = rand::rngs::StdRng::seed_from_u64(6678235);
    let result = Arc::new(Mutex::new((0.to_biguint().unwrap(), false, r)));
    let mut handles = vec![];

    for _ in 0..12 {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || loop {
            let mut mutex_guard = result.lock().unwrap();
            if mutex_guard.1 {
                break;
            }

            let candidate = mutex_guard.2.gen_biguint(1024);
            std::mem::drop(mutex_guard);
            let is_prime = miller_rabin(&candidate, &10);

            if is_prime {
                let mut mutex_guard = result.lock().unwrap();
                mutex_guard.0 = candidate;
                mutex_guard.1 = true;
                break;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("n: {}", (*result.lock().unwrap()).0);

    let a = 60.to_bigint().unwrap();
    let b = 13.to_bigint().unwrap();
    let gcd = extended_euclid(a, b);
    println!("x: {}", gcd.0);
    println!("y: {}", gcd.1);
    println!("gcd: {}", gcd.2);
}

/*
fn find_prime() -> (BigUint, u32) {
    let mut rng = rand::thread_rng();
    let bit_size = 2048;
    let mut candidate = rng.gen_biguint(bit_size);
    let mut trials = 0;
    while !miller_rabin(&candidate, &10) {
        candidate = rng.gen_biguint(bit_size);
        trials += 1;
    }
    return (candidate, trials);
}
*/

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

fn extended_euclid(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt){
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_x = 1.to_bigint().unwrap();
    let mut x = 0.to_bigint().unwrap();
    let mut old_y = 0.to_bigint().unwrap();
    let mut y = 1.to_bigint().unwrap();
    while r != BigInt::zero() {
        let q = old_r.clone() / r.clone();
        (old_r, r) = (r.clone(), old_r - &q * r);
        (old_x, x) = (x.clone(), old_x - &q * x);
        (old_y, y) = (y.clone(), old_y - &q * y);
    }
    return (old_x, old_y, old_r)
}