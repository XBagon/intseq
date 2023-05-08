pub mod closure_version {
    pub use btree::CachedIntegerSequenceComputer;
    pub struct IntegerSequence<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> {
        definition: D,
    }

    impl<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> IntegerSequence<D> {
        pub fn from(definition: D) -> Self {
            Self {
                definition,
            }
        }

        pub fn at(&self, n: u32) -> i32 {
            (self.definition.clone())(&mut |n| self.at(n), n)
        }
    }

    #[deprecated]
    pub mod vec {
        pub use super::IntegerSequence;

        #[ignore]
        #[test]
        fn test() {
            let mut fib = IntegerSequence::from(|f: &mut dyn FnMut(u32) -> i32, n| match n {
                0 => 0,
                1 => 1,
                _ => f(n - 2) + f(n - 1),
            });
            let f_n = fib.at(35);

            let mut cached_computer = CachedIntegerSequenceComputer::from(fib);
            let fc_n = cached_computer.at(35);

            assert_eq!(f_n, fc_n);
        }
        pub struct CachedIntegerSequenceComputer<
            D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone,
        > {
            cache: Vec<i32>,
            integer_sequence: IntegerSequence<D>,
        }

        impl<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> CachedIntegerSequenceComputer<D> {
            pub fn from(integer_sequence: IntegerSequence<D>) -> Self {
                Self {
                    cache: Vec::new(),
                    integer_sequence,
                }
            }

            pub fn at(&mut self, n: u32) -> i32 {
                if self.cache.len() > n as usize {
                    self.cache[n as usize]
                } else {
                    let n_0 = (self.integer_sequence.definition.clone())(&mut |n| self.at(n), n);
                    debug_assert_eq!(self.cache.len(), n as usize);
                    self.cache.push(n_0);
                    n_0
                }
            }
        }
    }

    pub mod btree {
        use std::collections::BTreeMap;

        pub use super::IntegerSequence;

        #[test]
        fn test() {
            let mut fib = IntegerSequence::from(|f: &mut dyn FnMut(u32) -> i32, n| match n {
                0 => 0,
                1 => 1,
                _ => f(n - 2) + f(n - 1),
            });

            let f_n = fib.at(35);

            let mut cached_computer = CachedIntegerSequenceComputer::from(fib);
            let fc_n = cached_computer.at(35);

            assert_eq!(f_n, fc_n);
        }
        pub struct CachedIntegerSequenceComputer<
            D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone,
        > {
            cache: BTreeMap<u32, i32>,
            integer_sequence: IntegerSequence<D>,
        }

        impl<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> CachedIntegerSequenceComputer<D> {
            pub fn from(integer_sequence: IntegerSequence<D>) -> Self {
                Self {
                    cache: BTreeMap::new(),
                    integer_sequence,
                }
            }

            pub fn at(&mut self, n: u32) -> i32 {
                if let Some(cached) = self.cache.get(&n) {
                    *cached
                } else {
                    let f_n = (self.integer_sequence.definition.clone())(&mut |n| self.at(n), n);
                    self.cache.insert(n, f_n);
                    f_n
                }
            }
        }
    }
    pub mod hashmap {
        use std::collections::HashMap;

        pub use super::IntegerSequence;

        #[test]
        fn test() {
            let mut fib = IntegerSequence::from(|f: &mut dyn FnMut(u32) -> i32, n| match n {
                0 => 0,
                1 => 1,
                _ => f(n - 2) + f(n - 1),
            });

            let f_n = fib.at(35);

            let mut cached_computer = CachedIntegerSequenceComputer::from(fib);
            let fc_n = cached_computer.at(35);

            assert_eq!(f_n, fc_n);
        }
        pub struct CachedIntegerSequenceComputer<
            D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone,
        > {
            cache: HashMap<u32, i32>,
            integer_sequence: IntegerSequence<D>,
        }

        impl<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> CachedIntegerSequenceComputer<D> {
            pub fn from(integer_sequence: IntegerSequence<D>) -> Self {
                Self {
                    cache: HashMap::new(),
                    integer_sequence,
                }
            }

            pub fn at(&mut self, n: u32) -> i32 {
                if let Some(cached) = self.cache.get(&n) {
                    *cached
                } else {
                    let f_n = (self.integer_sequence.definition.clone())(&mut |n| self.at(n), n);
                    self.cache.insert(n, f_n);
                    f_n
                }
            }
        }
    }
    pub mod indexmap {
        use indexmap::IndexMap;

        pub use super::IntegerSequence;

        #[test]
        fn test() {
            let mut fib = IntegerSequence::from(|f: &mut dyn FnMut(u32) -> i32, n| match n {
                0 => 0,
                1 => 1,
                _ => f(n - 2) + f(n - 1),
            });

            let f_n = fib.at(35);

            let mut cached_computer = CachedIntegerSequenceComputer::from(fib);
            let fc_n = cached_computer.at(35);

            assert_eq!(f_n, fc_n);
        }
        pub struct CachedIntegerSequenceComputer<
            D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone,
        > {
            cache: IndexMap<u32, i32>,
            integer_sequence: IntegerSequence<D>,
        }

        impl<D: FnMut(&mut dyn FnMut(u32) -> i32, u32) -> i32 + Clone> CachedIntegerSequenceComputer<D> {
            pub fn from(integer_sequence: IntegerSequence<D>) -> Self {
                Self {
                    cache: IndexMap::new(),
                    integer_sequence,
                }
            }

            pub fn at(&mut self, n: u32) -> i32 {
                if let Some(cached) = self.cache.get(&n) {
                    *cached
                } else {
                    let f_n = (self.integer_sequence.definition.clone())(&mut |n| self.at(n), n);
                    self.cache.insert(n, f_n);
                    f_n
                }
            }
        }
    }
}

pub mod trait_version {
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let f_n = FibonacciSequence::at(35);

        let mut cached_computer = CachedIntegerSequenceComputer::from_is(FibonacciSequence);
        let fc_n = cached_computer.at(35);

        assert_eq!(f_n, fc_n);
    }

    pub struct FibonacciSequence;

    impl IntegerSequence for FibonacciSequence {
        fn calculate(mut f: impl FnMut(u32) -> i32, n: u32) -> i32 {
            match n {
                0 => 0,
                1 => 1,
                _ => f(n - 2) + f(n - 1),
            }
        }
    }

    pub trait IntegerSequence {
        fn calculate(f: impl FnMut(u32) -> i32, n: u32) -> i32;
        fn at(n: u32) -> i32 {
            Self::calculate(|n| Self::at(n), n)
        }
    }

    pub struct CachedIntegerSequenceComputer<IS: IntegerSequence> {
        integer_sequence: IS,
        cache: BTreeMap<u32, i32>,
    }

    impl<IS: IntegerSequence> CachedIntegerSequenceComputer<IS> {
        pub fn at(&mut self, n: u32) -> i32 {
            if let Some(cached) = self.cache.get(&n) {
                *cached
            } else {
                let f_n = <IS as IntegerSequence>::calculate(|n| self.at(n), n);
                self.cache.insert(n, f_n);
                f_n
            }
        }

        pub fn from_is(integer_sequence: IS) -> Self {
            Self {
                integer_sequence,
                cache: BTreeMap::new(),
            }
        }
    }
}
