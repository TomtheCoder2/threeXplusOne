use std::time::Instant;

struct Checker {
    checked: Vec<bool>,
}

impl Checker {
    fn new(n: usize) -> Self {
        Checker {
            checked: vec![false; n]
        }
    }

    fn check(&mut self, mut n: u32) -> bool {
        match n {
            0 | 1 | 2 | 4 => true,
            _ => {
                while n & (1 << 0) == 0 {
                    n /= 2;
                }
                if n == 1 || (self.checked.len() > n as usize && self.checked[n as usize]) {
                    true
                } else if self.checked.len() > n as usize {
                    let r = self.check(3 * n + 1);
                    self.checked[n as usize] = r;
                    r
                } else {
                    self.check(3 * n + 1)
                }
            }
        }
    }
}

// without cache: 4ns for 100
// with cache: 71ns for 100
fn main() {
    let n = 27;
    let mut checker = Checker::new(100000);
    let n = 1000000;
    benchmark(move |i| {
        // println!("checking: {}", i);
        checker.check(i as u32);
    }, n as usize);
    let mut checker = Checker::new(100000);

    let start = Instant::now();
    for i in 0..n {
        checker.check(i);
    }
    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    println!("Average time: {:?}", duration / n);

    println!("check n={} -> {}", n, checker.check(n));
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
