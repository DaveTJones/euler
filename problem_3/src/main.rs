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
