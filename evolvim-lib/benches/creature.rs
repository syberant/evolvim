#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::ecs_board::BoardSize;
    use self::lib_evolvim::*;
    use self::test::Bencher;

    const TEST_BOARD_SIZE: BoardSize = (100, 100);
    const TEST_TIME: f64 = 0.0;
    const TEST_ENERGY: f64 = 1.3;

    #[bench]
    fn bench_creature_new_baby_1_parent(b: &mut Bencher) {
        let mut world = nphysics2d::world::World::<f64>::new();

        let parents: Vec<SoftBody<Brain>> =
            vec![SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME)];

        let parents: Vec<&SoftBody<Brain>> = parents.iter().collect();

        b.iter(|| Creature::new_baby(&mut world, &parents, TEST_ENERGY, TEST_TIME));
    }

    #[bench]
    fn bench_creature_new_baby_2_parents(b: &mut Bencher) {
        let mut world = nphysics2d::world::World::<f64>::new();

        let parents: Vec<SoftBody<Brain>> = vec![
            SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME),
            SoftBody::new_random(&mut world, TEST_BOARD_SIZE, TEST_TIME),
        ];

        let parents: Vec<&SoftBody<Brain>> = parents.iter().collect();

        b.iter(|| Creature::new_baby(&mut world, &parents, TEST_ENERGY, TEST_TIME));
    }
}
