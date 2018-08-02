#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    const FRAME_TIME_STEP: f64 = 0.001;

    #[bench]
    fn bench_new_default(b: &mut Bencher) {
        b.iter(|| Board::default());
    }

    #[bench]
    fn bench_update(b: &mut Bencher) {
        let mut board = Board::default();

        b.iter(|| {
            board.update(FRAME_TIME_STEP);
        });
    }
}
