#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::constants::*;
    use self::lib_evolvim::{Climate, Terrain};
    use self::test::Bencher;

    #[bench]
    fn bench_terrain_perlin_generation(b: &mut Bencher) {
        b.iter(|| Terrain::generate_perlin(DEFAULT_BOARD_SIZE, DEFAULT_NOISE_STEP_SIZE));
    }

    #[bench]
    fn bench_terrain_update_all(b: &mut Bencher) {
        let mut time = 0.0;
        let mut terrain = Terrain::generate_perlin(DEFAULT_BOARD_SIZE, DEFAULT_NOISE_STEP_SIZE);
        let climate = Climate::new(DEFAULT_MIN_TEMP, DEFAULT_MAX_TEMP);

        b.iter(|| {
            time += 0.001;
            terrain.update_all(time, &climate);
        });
    }
}
