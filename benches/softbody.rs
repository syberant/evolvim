#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate lib_evolvim;
    extern crate test;

    use self::lib_evolvim::*;
    use self::test::Bencher;

    fn get_stable_870() -> Board {
        Board::load_from("assets/stable_870.bin").unwrap()
    }

    #[bench]
    fn bench_softbody_collide_all(b: &mut Bencher) {
        let board = get_stable_870();

        b.iter(|| {
            for c_rc in &board.creatures {
                c_rc.collide(&board.soft_bodies_in_positions);
            }
        });
    }
}
