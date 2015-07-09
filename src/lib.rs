extern crate num;

use num::traits::{ Zero, One, FromPrimitive, ToPrimitive };
use num::bigint::BigInt;

const DEFAULT_BUCKET_SIZE: usize = 5;

pub trait ModPow<T> {
    fn mod_pow(&self, exp: &T, m: &T) -> T;
    fn mod_pow_k(&self, exp: &T, m: &T, k: usize) -> T;
}

impl ModPow<BigInt> for BigInt {

    fn mod_pow(&self, exp: &BigInt, m: &BigInt) -> BigInt {
        self.mod_pow_k(exp, m, DEFAULT_BUCKET_SIZE)
    }

    fn mod_pow_k(&self, exp: &BigInt, m: &BigInt, k: usize) -> BigInt {

        let base = 2 << (k - 1);

        let mut table = Vec::with_capacity(base);
        table.push(BigInt::one());

        for i in 1..base {
            let last = table.get_mut(i-1).unwrap().clone();

            table.push((last * self) % m);
        }

        let mut r = BigInt::one();

        for i in digits_of_n(exp, base).iter().rev() {
            for _ in 0..k {
                r = &r * &r % m
            }

            if *i != 0 {
                r = &r * table.get(*i).unwrap() % m;
            }
        }

        r
    }
}

fn digits_of_n(e: &BigInt, b: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    let mut n = (*e).clone();
    let base = BigInt::from_usize(b).unwrap();

    while n > BigInt::zero() {
        digits.push((&n % &base).to_usize().unwrap());
        n = &n / &base;
    }

    digits
}
