#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate graphics;
    extern crate lib_evolvim;
    extern crate test;

    // use self::lib_evolvim::constants::*;
    use self::lib_evolvim::graphics::*;
    // use self::lib_evolvim::*;
    use self::test::Bencher;

    #[bench]
    fn bench_graphics_from_hsba(b: &mut Bencher) {
        let test_color = [1.0, 0.5, 0.3, 0.98];

        b.iter(|| from_hsba(test_color));
    }
}
