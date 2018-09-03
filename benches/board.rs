#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    const FRAME_TIME_STEP: f64 = 0.001;

    fn get_stable_870() -> Board {
        Board::load_from("assets/stable_870.bin").unwrap()
    }

    #[bench]
    fn bench_board_new_default(b: &mut Bencher) {
        b.iter(|| Board::default());
    }

    #[bench]
    fn bench_board_update_870(b: &mut Bencher) {
        let mut board = get_stable_870();

        b.iter(|| {
            board.update(FRAME_TIME_STEP);
        });
    }
}
