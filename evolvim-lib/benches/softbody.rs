#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    // use self::lib_evolvim::Brain;
    // use self::test::Bencher;

    // const TIME_STEP: f64 = 0.001;

}
