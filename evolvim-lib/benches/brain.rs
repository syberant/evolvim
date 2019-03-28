#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    // const TEST_INPUT: BrainInput = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
    const TEST_BOARD_SIZE: BoardSize = (100, 100);
    const TEST_TIME: f64 = 0.0;

    fn get_test_board() -> Board {
        Board::load_from("assets/test.bin").unwrap()
    }

    #[bench]
    fn bench_brain_new_random(b: &mut Bencher) {
        b.iter(|| Brain::new_random());
    }

    #[bench]
    fn bench_brain_run_with(b: &mut Bencher) {
        let mut board = get_test_board();
        let creature: &mut SoftBody = &mut board.creatures[0].borrow_mut();
        let brain = &mut creature.brain;
        let env = Environment::new(&board.terrain, &creature.base);

        b.iter(|| {
            brain.run_with(&env);
        });
    }

    #[bench]
    fn bench_brain_load_input(b: &mut Bencher) {
        let mut board = get_test_board();
        let creature: &mut SoftBody = &mut board.creatures[0].borrow_mut();
        let brain = &mut creature.brain;
        let env = Environment::new(&board.terrain, &creature.base);

        b.iter(|| brain.load_input(&env));
    }

    #[bench]
    fn bench_brain_feed_forward(b: &mut Bencher) {
        let mut board = get_test_board();
        let creature: &mut SoftBody = &mut board.creatures[0].borrow_mut();
        let brain = &mut creature.brain;
        let env = Environment::new(&board.terrain, &creature.base);

        brain.load_input(&env);

        b.iter(|| brain.run());
    }

    #[bench]
    fn bench_brain_evolve_1_parent(b: &mut Bencher) {
        let parents = vec![HLSoftBody::from(SoftBody::new_random(
            TEST_BOARD_SIZE,
            TEST_TIME,
        ))];

        b.iter(|| Brain::recombination_infinite_parents(&parents));
    }

    #[bench]
    fn bench_brain_evolve_2_parents(b: &mut Bencher) {
        let parents = vec![
            HLSoftBody::from(SoftBody::new_random(TEST_BOARD_SIZE, TEST_TIME)),
            HLSoftBody::from(SoftBody::new_random(TEST_BOARD_SIZE, TEST_TIME)),
        ];

        b.iter(|| Brain::recombination_infinite_parents(&parents));
    }
}
