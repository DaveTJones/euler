// Define fibbonaci struct with upper bound to determine highest returnable value
struct Fib {
    current: u64,
    next: u64,
}

// only interested in even values, so initialise with first two even Fibbonaci numbers
impl Fib {
    fn new() -> Fib {
        Fib {
            current: 2,
            next: 8,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        // modified Fibbonaci formula to return only sequential even terms
        self.current = self.next;
        self.next = 4 * self.next + current;
        Some(current)
    }
}

fn main() {
    let bound = 4_000_000;
    let test: u64 = Fib::new().take_while(|x| x < &bound).sum();
    println!("{test:?}");
}
