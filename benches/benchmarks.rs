use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};

fn simple_fib(n: u32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        n => simple_fib(n - 1) + simple_fib(n - 2),
    }
}

struct SimpleCachedFib {
    cache: Vec<i32>,
}
impl SimpleCachedFib {
    pub fn new() -> Self {
        Self {
            cache: vec![0, 1],
        }
    }
}

impl SimpleCachedFib {
    pub fn at(&mut self, n: u32) -> i32 {
        self.cache.reserve_exact(n as usize + 1);
        if self.cache.len() > n as usize {
            self.cache[n as usize]
        } else {
            let f_n_1 = self.at(n - 1);
            let f_n_2 = self.at(n - 2);
            let f_n = f_n_1 + f_n_2;
            debug_assert!(self.cache.len() == n as usize);
            self.cache.push(f_n);
            f_n
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_cached_fib() {
        panic!();
        let mut scf = SimpleCachedFib::new();
        debug_assert_eq!(SimpleCachedFib::new().at(24), simple_fib(24));
    }
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibs");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for input in [1, 4, 8, 16, 24, 32] {
        group.bench_with_input(BenchmarkId::new("simple_naive_fib", input), &input, |b, &input| {
            b.iter(|| simple_fib(black_box(input)));
        });
        group.bench_with_input(
            BenchmarkId::new("simple_cached_fib", input),
            &input,
            |b, &input| {
                b.iter(|| SimpleCachedFib::new().at(black_box(input)));
            },
        );
        group.bench_with_input(BenchmarkId::new("trait_naive_fib", input), &input, |b, &input| {
            use intseq::trait_version::*;
            b.iter(|| FibonacciSequence::at(black_box(input)));
        });
        group.bench_with_input(BenchmarkId::new("trait_cached_fib", input), &input, |b, &input| {
            use intseq::trait_version::*;
            b.iter(|| {
                CachedIntegerSequenceComputer::from_is(FibonacciSequence).at(black_box(input))
            });
        });
        group.bench_with_input(
            BenchmarkId::new("closure_naive_fib", input),
            &input,
            |b, &input| {
                use intseq::closure_version::*;
                b.iter(|| {
                    IntegerSequence::from(|f: &mut dyn FnMut(u32) -> i32, n| match n {
                        0 => 0,
                        1 => 1,
                        _ => f(n - 2) + f(n - 1),
                    })
                    .at(black_box(input))
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("closure_cached_fib", input),
            &input,
            |b, &input| {
                use intseq::closure_version::*;
                b.iter(|| {
                    CachedIntegerSequenceComputer::from(IntegerSequence::from(
                        |f: &mut dyn FnMut(u32) -> i32, n| match n {
                            0 => 0,
                            1 => 1,
                            _ => f(n - 2) + f(n - 1),
                        },
                    ))
                    .at(black_box(input))
                });
            },
        );
    }
    group.finish();
}

fn bench_closure_cached_fib_caches(c: &mut Criterion) {
    let mut group = c.benchmark_group("closure_cached_fib_caches");
    for input in [1, 4, 8, 16, 24, 32] {
        group.bench_with_input(BenchmarkId::new("vec", input), &input, |b, &input| {
            use intseq::closure_version::vec::*;
            b.iter(|| {
                CachedIntegerSequenceComputer::from(IntegerSequence::from(
                    |f: &mut dyn FnMut(u32) -> i32, n| match n {
                        0 => 0,
                        1 => 1,
                        _ => f(n - 2) + f(n - 1),
                    },
                ))
                .at(black_box(input))
            });
        });
        group.bench_with_input(BenchmarkId::new("btree", input), &input, |b, &input| {
            use intseq::closure_version::btree::*;
            b.iter(|| {
                CachedIntegerSequenceComputer::from(IntegerSequence::from(
                    |f: &mut dyn FnMut(u32) -> i32, n| match n {
                        0 => 0,
                        1 => 1,
                        _ => f(n - 2) + f(n - 1),
                    },
                ))
                .at(black_box(input))
            });
        });
        group.bench_with_input(BenchmarkId::new("hashmap", input), &input, |b, &input| {
            use intseq::closure_version::hashmap::*;
            b.iter(|| {
                CachedIntegerSequenceComputer::from(IntegerSequence::from(
                    |f: &mut dyn FnMut(u32) -> i32, n| match n {
                        0 => 0,
                        1 => 1,
                        _ => f(n - 2) + f(n - 1),
                    },
                ))
                .at(black_box(input))
            });
        });
        group.bench_with_input(BenchmarkId::new("indexmap", input), &input, |b, &input| {
            use intseq::closure_version::indexmap::*;
            b.iter(|| {
                CachedIntegerSequenceComputer::from(IntegerSequence::from(
                    |f: &mut dyn FnMut(u32) -> i32, n| match n {
                        0 => 0,
                        1 => 1,
                        _ => f(n - 2) + f(n - 1),
                    },
                ))
                .at(black_box(input))
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs, bench_closure_cached_fib_caches);
criterion_main!(benches);
