struct Fib {
    current: u64,
    next: u64,
    bound: u64,
}

impl Fib {
    fn new() -> Fib {
        Fib {
            current: 2,
            next: 8,
            bound: 4_000_000,
        }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bound < self.current {
            return None;
        }
        let current = self.current;
        self.current = self.next;
        self.next = 4 * self.next + current;
        Some(current)
    }
}

fn main() {
    let test: u64 = Fib::new().take(103).sum();
    println!("{test:?}");
}
