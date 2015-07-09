extern crate num;
extern crate mod_pow;

use num::bigint::{ BigInt, Sign };
use num::traits::FromPrimitive;

use mod_pow::ModPow;

#[test]
fn test_bigint_mod_pow() {
    fn check(b: &BigInt, e: &BigInt, m: &BigInt, r: &BigInt) {
        assert_eq!(b.mod_pow(&e, &m), *r);
    }

    fn check_i64(b: i64, e: i64, m: i64, r: i64) {
        let big_b = BigInt::from_i64(b).unwrap();
        let big_e = BigInt::from_i64(e).unwrap();
        let big_m = BigInt::from_i64(m).unwrap();
        let big_r = BigInt::from_i64(r).unwrap();

        check(&big_b, &big_e, &big_m, &big_r);
    }


    check_i64(-2, 5, 33, -32);
    check_i64(-2, 5, 32, 0);
    check_i64(-1, 3, 10, -1);
    check_i64(-1, 4, 10, 1);
    check_i64(0, 2352, 21, 0);
    check_i64(1, 26, 21, 1);
    check_i64(2, 5, 33, 32);
    check_i64(2, 5, 32, 0);
    check_i64(std::i64::MAX, std::i64::MAX, 2, 1);

    let big_b = BigInt::new(Sign::Plus, vec![12,234,234,556,34]);
    let big_e = BigInt::new(Sign::Plus, vec![12,234,234,556,34]);
    let big_m = BigInt::new(Sign::Plus, vec![234,556,34]);
    let big_r = BigInt::new(Sign::Plus, vec![2689017340, 2002504038, 5]);

    check(&big_b, &big_e, &big_m, &big_r);
}
