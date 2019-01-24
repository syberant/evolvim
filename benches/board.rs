#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    const FRAME_TIME_STEP: f64 = 0.001;

    fn get_test_board() -> Board {
        Board::load_from("assets/test.bin").unwrap()
    }

    #[bench]
    fn bench_board_new_default(b: &mut Bencher) {
        b.iter(|| Board::default());
    }

    #[bench]
    fn bench_board_update(b: &mut Bencher) {
        let mut board = get_test_board();

        b.iter(|| {
            board.update(FRAME_TIME_STEP);
        });
    }
}
