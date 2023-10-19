#![allow(unused)]

// use rand::Rng;

struct Primes {
    target: i64,
    primes: Vec<i64>,
    factors: Vec<i64>,
    current: i64,
}

impl Primes {
    fn new(target: i64) -> Primes {
        Primes {
            target,
            primes: Vec::new(),
            factors: Vec::new(),
            current: 3,
        }
    }
}

impl Iterator for Primes {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let sol: i64;
        if self.primes.len() == 0 {
            // Unique handling of 2 to as first prime
            sol = 2;
        } else {
            // run prime check
            while self.primes.iter().any(|n| self.current % n == 0) {
                self.current += 2;
            }
            sol = self.current;
        }

        self.primes.push(sol);
        Some(sol)
    }
}

// Input #1: n > 2, an odd integer to be tested for primality
// Input #2: k, the number of rounds of testing to perform
// Output: “composite” if n is found to be composite, “probably prime” otherwise

// let s > 0 and d odd > 0 such that n − 1 = 2sd  # by factoring out powers of 2 from n − 1
// repeat k times:
//     a ← random(2, n − 2)  # n is always a probable prime to base 1 and n − 1
//     x ← ad mod n
//     repeat s times:
//         y ← x2 mod n
//         if y = 1 and x ≠ 1 and x ≠ n − 1 then # nontrivial square root of 1 modulo n
//             return “composite”
//         x ← y
//     if y ≠ 1 then
//         return “composite”
// return “probably prime”

// fn primality_test(n: i64) -> Result<bool, bool> {
//     const K: i64 = 2;

//     let n_prime = n - 1;

//     let mut s = 1;

//     let mut d = n_prime / (2 * s);

//     while d % 2 == 0 {
//         s = s * 2;
//         d = n_prime / (2 * s);
//     }

//     Ok(true)
// }

fn main() {
    let mut target = 600851475143_i64;
    let mut factor = 3;
    while target != 1 {
        if target % factor != 0 {
            factor += 2;
        } else {
            target /= factor;
        }
    }
    println!("{factor}")
}
