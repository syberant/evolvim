include!(concat!(env!("OUT_DIR"), "/sine_approx.rs"));

#[cfg(test)]
mod tests {
    extern crate rand;

    #[test]
    fn test_lookup_sine_approx() {
        use self::rand::thread_rng;
        use self::rand::Rng;

        const TEST_PRECISION: usize = 10000;

        let mut rng = thread_rng();

        for _i in 0..TEST_PRECISION {
            let to_test: f64 = (rng.gen::<f64>() * 2.0 - 1.0) * 100000.0;

            let approx = super::lookup_sine_approx(to_test);
            let real = to_test.sin();
            let diff = (real - approx).abs();

            assert!(
                diff < 0.001_f64,
                "Testing {}: Real is {}, approximation is {}, difference is {}",
                to_test,
                real,
                approx,
                diff
            );
        }
    }
}