#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::constants::*;
    use self::lib_evolvim::Rock;
    use self::test::Bencher;

    #[bench]
    fn bench_rock_new_random(b: &mut Bencher) {
        let energy = 1.0;

        b.iter(|| Rock::new_random(DEFAULT_BOARD_SIZE, ROCK_DENSITY, energy));
    }
}
