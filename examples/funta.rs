use std::collections::HashSet;

use indexmap::IndexSet;
use intseq::closure_version::{CachedIntegerSequenceComputer, IntegerSequence};

fn main() {
    let funta = IntegerSequence::from(|f, n| match n {
        0 => 1,
        _ => (0..n - 1)
            .map(|n| {
                let f_n = f(n);
                if (f_n as u32) <= n {
                    f_n
                } else {
                    -f_n
                }
            })
            .sum(),
    });

    let mut set = IndexSet::new();

    let mut funta = CachedIntegerSequenceComputer::from(funta);
    for n in 0..100000 {
        let funta_n = funta.at(n);
        //println!("funta({}) = {}", n, funta_n);
        set.insert(funta_n);
    }
    println!("{:?}", set);
    //just scrambled fibonacci numbers
    // 1, 0, -1, 2, 3, 5, 8, 13, 21, -13, -5, 34, -21, -8, 55, -34, 89, -55, 144, -89, 233, -144, 377, -233, 610, -377, 987, -610, 1597, -987, 2584, -1597, 4181, -2584, 6765, -4181, 10946, -6765, 17711, -10946, 28657, -17711, 46368, -28657, 75025, -46368, 121393, -75025, 196418, -121393
}
