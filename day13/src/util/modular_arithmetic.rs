use std::ops::Sub;

use num_traits::{one, zero, PrimInt, Signed, Unsigned};

struct ExtendedGcdOutput<T: PrimInt + Signed> {
    bezout_coeffs: (T, T),
    gcd: T,
    quotients: (T, T),
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd<T: PrimInt + Signed>(a: T, b: T) -> ExtendedGcdOutput<T> {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (one(), zero());
    let (mut old_t, mut t) = (zero(), one());

    while !r.is_zero() {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    ExtendedGcdOutput {
        bezout_coeffs: (old_s, old_t),
        gcd: old_r,
        quotients: (t, s),
    }
}

// bezout: s_k * a + t_k * b = r_k
//

// function inverse(a, n)
//     t := 0;     newt := 1
//     r := n;     newr := a

//     while newr ≠ 0 do
//         quotient := r div newr
//         (t, newt) := (newt, t − quotient × newt)
//         (r, newr) := (newr, r − quotient × newr)

//     if r > 1 then
//         return "a is not invertible"
//     if t < 0 then
//         t := t + n

//     return t
