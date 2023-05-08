use intseq::closure_version::{CachedIntegerSequenceComputer, IntegerSequence};

/// https://oeis.org/A130618
fn main() {
    let adjwindow = IntegerSequence::from(|f, n| match n {
        0 => 1,
        _ => {
            let f_n_1 = f(n - 1);
            let window_size = f_n_1 as u32 % (n);
            (n - 1 - window_size as u32..n).map(|n| f(n)).sum()
        }
    });
    let mut adjwindow = CachedIntegerSequenceComputer::from(adjwindow);
    for n in 0..100 {
        println!("adjwindow({}) = {}", n, adjwindow.at(n));
    }
}
