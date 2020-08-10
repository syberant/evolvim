#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate nphysics2d;
    extern crate test;

    use self::lib_evolvim::ecs_board::BoardSize;
    use self::lib_evolvim::{Brain, SoftBody};
    use self::lib_evolvim::{GenerateRandom, RecombinationInfinite};
    use self::test::Bencher;

    // const TEST_INPUT: BrainInput = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
    const TEST_BOARD_SIZE: BoardSize = (100, 100);
    const TEST_TIME: f64 = 0.0;

    #[bench]
    fn bench_brain_new_random(b: &mut Bencher) {
        b.iter(|| Brain::new_random());
    }

    #[bench]
    fn bench_brain_evolve_1_parent(b: &mut Bencher) {
        let mut world = nphysics2d::world::World::<f64>::new();

        let parents = vec![SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME)];

        let parents: Vec<&SoftBody<Brain>> = parents.iter().collect();

        b.iter(|| Brain::recombination_infinite_parents(&parents));
    }

    #[bench]
    fn bench_brain_evolve_2_parents(b: &mut Bencher) {
        let mut world = nphysics2d::world::World::<f64>::new();

        let parents = vec![
            SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME),
            SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME),
        ];

        let parents: Vec<&SoftBody<Brain>> = parents.iter().collect();

        b.iter(|| Brain::recombination_infinite_parents(&parents));
    }
}
