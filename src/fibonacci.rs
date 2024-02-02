pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn fibonacci() {
    let fib = Fibonacci::default();
    for i in fib {
        print!("{} ", i);
    }
}
