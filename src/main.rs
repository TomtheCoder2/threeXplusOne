use std::time::Instant;

struct Checker {
    checked: Vec<bool>,
    checked_min: usize,
}

impl Checker {
    fn new(n: usize) -> Self {
        Checker {
            checked: vec![false; n],
            checked_min: 0,
        }
    }

    fn check(&mut self, n: u32) -> bool {
        let r = self.check_internal(n);
        if !r {
            println!(
                "found a number that doesnt return to the normal 1-2-4 loop: {}",
                n
            );
            false
        } else if self.checked.len() > n as usize - self.checked_min {
            for _ in 0..n as usize - self.checked_min {
                self.checked.remove(0);
            }
            self.checked_min = n as usize;
            true
        } else {
            true
        }
    }

    fn check_internal(&mut self, mut n: u32) -> bool {
        match n {
            0 | 1 | 2 | 4 => true,
            _ => {
                while n & (1 << 0) == 0 {
                    n /= 2;
                }
                if n == 1
                    || (n as usize > self.checked_min
                        && self.checked.len() > n as usize - self.checked_min
                        && self.checked[n as usize - self.checked_min])
                    || n < self.checked_min as u32
                {
                    true
                } else if self.checked.len() > n as usize - self.checked_min {
                    let r = self.check_internal(3 * n + 1);
                    self.checked[n as usize - self.checked_min] = r;
                    r
                } else {
                    self.check_internal(3 * n + 1)
                }
            }
        }
    }
}

// without cache: 4ns for 100
// with cache: 71ns for 100
fn main() {
    let _n = 27;
    let mut checker = Checker::new(10000);
    let n = 10000;
    benchmark(
        move |i| {
            // println!("checking: {}", i);
            checker.check(i as u32);
        },
        n as usize,
    );
    let mut checker = Checker::new(100000);

    let start = Instant::now();
    for i in 0..n {
        checker.check(i);
    }
    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    println!("Average time: {:?}", duration / n);

    println!("check n={} -> {}", n, checker.check_internal(n));
}

/// Benchmark a function
/// # Arguments:
/// * `f`(false, n)
/// * `n` - The number of times to run the function
pub fn benchmark<F>(mut f: F, n: usize)
where
    F: FnMut(usize) + Send + 'static,
{
    let start = Instant::now();
    for i in 0..n {
        f(i);
    }
    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    println!("Average time: {:?}", duration / n as u32);
}
