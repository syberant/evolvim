#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    const TEST_INPUT: BrainInput = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
    const TEST_BOARD_SIZE: BoardSize = (100, 100);
    const TEST_TIME: f64 = 0.0;

    #[bench]
    fn bench_brain_new_random(b: &mut Bencher) {
        b.iter(|| Brain::new_random());
    }

    #[bench]
    fn bench_brain_run(b: &mut Bencher) {
        let mut brain = Brain::new_random();

        b.iter(|| {
            brain.run(TEST_INPUT);
        });
    }

    #[bench]
    fn bench_brain_load_input(b: &mut Bencher) {
        let mut brain = Brain::new_random();

        b.iter(|| brain.load_input(TEST_INPUT));
    }

    #[bench]
    fn bench_brain_feed_forward(b: &mut Bencher) {
        let mut brain = Brain::new_random();
        brain.load_input(TEST_INPUT);

        b.iter(|| brain.feed_forward());
    }

    #[bench]
    fn bench_brain_evolve_1_parent(b: &mut Bencher) {
        let parents = vec![HLSoftBody::from(SoftBody::new_random_creature(
            TEST_BOARD_SIZE,
            TEST_TIME,
        ))];

        b.iter(|| Brain::evolve(&parents));
    }

    #[bench]
    fn bench_brain_evolve_2_parents(b: &mut Bencher) {
        let parents = vec![
            HLSoftBody::from(SoftBody::new_random_creature(TEST_BOARD_SIZE, TEST_TIME)),
            HLSoftBody::from(SoftBody::new_random_creature(TEST_BOARD_SIZE, TEST_TIME)),
        ];

        b.iter(|| Brain::evolve(&parents));
    }
}
