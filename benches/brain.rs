#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::{Brain, BrainInput};
    use self::test::Bencher;

    const TEST_INPUT: BrainInput = [1., 2., 3., 4., 5., 6., 7., 8., 9.];

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
}
