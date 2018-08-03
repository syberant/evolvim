#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::constants::*;
    use self::lib_evolvim::Climate;
    use self::test::Bencher;

    #[bench]
    fn bench_climate_new(b: &mut Bencher) {
        b.iter(|| Climate::new(DEFAULT_MIN_TEMP, DEFAULT_MAX_TEMP));
    }

    #[bench]
    fn bench_climate_update(b: &mut Bencher) {
        let mut climate = Climate::new(DEFAULT_MIN_TEMP, DEFAULT_MAX_TEMP);
        let mut time = 0.0;

        b.iter(|| {
            climate.update(time);
            time += 0.001;
        });
    }

    #[bench]
    fn bench_climate_get_growth_over_time_range(b: &mut Bencher) {
        let climate = Climate::new(DEFAULT_MIN_TEMP, DEFAULT_MAX_TEMP);
        let mut time = 0.01;
        let mut last_updated = 0.0;

        b.iter(|| {
            time += 0.001;
            last_updated += 0.001;

            // Force the compiler to calculate it.
            return climate.get_growth_over_time_range(time, last_updated);
        })
    }
}
