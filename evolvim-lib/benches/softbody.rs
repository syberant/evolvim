#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::{Board, Brain};
    use self::test::Bencher;

    const TIME_STEP: f64 = 0.001;

    fn get_test_board() -> Board {
        Board::<Brain>::load_from("assets/test.bin").unwrap()
    }

    #[bench]
    fn bench_softbody_collide(b: &mut Bencher) {
        let board = get_test_board();

        b.iter(|| {
            for c_rc in &board.creatures {
                c_rc.collide(&board.soft_bodies_in_positions);
            }
        });
    }

    #[bench]
    fn bench_softbody_metabolize(b: &mut Bencher) {
        let board = get_test_board();

        b.iter(|| {
            let time = board.get_time();

            for c_rc in &board.creatures {
                c_rc.borrow_mut().metabolize(TIME_STEP, time);
            }
        });
    }

    #[bench]
    fn bench_softbody_apply_motions(b: &mut Bencher) {
        let mut board = get_test_board();

        b.iter(|| {
            let board_size = board.get_board_size();

            for c_rc in &board.creatures {
                c_rc.apply_motions(
                    TIME_STEP * 100.0,
                    board_size,
                    &board.terrain,
                    &mut board.soft_bodies_in_positions,
                );
            }
        });
    }
}
