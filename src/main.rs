use std::time::Instant;

struct Checker {
    checked: Vec<bool>,
    checked_min: usize,
}

static mut USE_CACHE: bool = false;

impl Checker {
    fn new(n: usize) -> Self {
        Checker {
            checked: vec![false; n],
            checked_min: 0,
        }
    }

    fn check(&mut self, n: u32) -> bool {
        if unsafe { USE_CACHE } {
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
        } else {
            self.check_internal(n)
        }
    }

    fn check_internal(&mut self, mut n: u32) -> bool {
        match n {
            0 | 1 | 2 | 4 => true,
            _ => {
                while n & (1 << 0) == 0 {
                    n /= 2;
                }
                if unsafe { USE_CACHE } {
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
    // get args to determine if we use the cache
    // args: ./executable [--cache] [--cache-size=10000] [n]
    let mut n = 10000;
    let mut cache_size = 10000;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        for arg in args.iter() {
            if arg == "--cache" {
                unsafe { USE_CACHE = true };
            } else if arg.starts_with("--cache-size=") {
                let size = arg.split('=').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
                unsafe { USE_CACHE = true };
                cache_size = size;
            } else if arg.starts_with("--n=") {
                let size = arg.split('=').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
                n = size;
            }
        }
    }
    println!("cache: {}", unsafe { USE_CACHE });
    println!("cache size: {}", cache_size);
    println!("n: {}", n);
    let mut checker = Checker::new(cache_size);
    benchmark(
        move |i| {
            // println!("checking: {}", i);
            checker.check(i as u32);
        },
        n as usize,
    );
    let mut checker = Checker::new(cache_size);
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
