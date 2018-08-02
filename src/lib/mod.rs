//! Evolv.io improved!
//!
//! This project is a Rust port of the original [Processing code](https://github.com/evolvio/evolv.io/tree/120b3c1f11c6beade92343fc40f57d376b8a7434) from [carykh](https://www.youtube.com/user/carykh).
//!
//! # Why is this improved?
//!
//! Because it's in Rust of course!
//!
//! There are (or will be: WIP) a number of reasons why this version is better than the original:
//! - WIP: better documentation
//! - Performance, [Processing](https://www.processing.org) isn't known for anything resembling speed, at least not when compared to Rust.
//!
//! The original version has since also [been converted](https://github.com/evolvio/evolv.io/) to Java. (No, I am not going to provide a link; Java isn't worthy of that)
//! This version still provides:
//! - a better language (If you haven't noticed: I hate Java)
//! - (very probably) better performance plus the option to turn off the graphics
//! - (hopefully) less weird bugs because of Rust's capability of safe memory management
//! - probably more but I AM NOT LOOKING AT JAVA CODE TO COMPARE
//!
//! # Performance
//!
//! ## Running benchmarks
//! I have written some benchmarks to assess the performance of this crate.
//! You can run them with `cargo bench --features=bench`.
//! Please note that this only works in nightly (`rustup default nightly` to use it by default).
//! Type `rustup toolchain install nightly` to install the latest nightly build; `rustup toolchain uninstall nightly` to uninstall again.
//!
//! ## My results
//! Just an example:
//! ```norun
//!      Running target/release/deps/board-5061053384a77075
//!
//! running 2 tests
//! test benches::bench_new_default ... bench:  10,985,787 ns/iter (+/- 3,969,594)
//! test benches::bench_update      ... bench:      74,712 ns/iter (+/- 43,579)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out
//!
//!      Running target/release/deps/brain-428e2aee350f3bc3
//!
//! running 4 tests
//! test benches::bench_feed_forward ... bench:         406 ns/iter (+/- 96)
//! test benches::bench_load_input   ... bench:          64 ns/iter (+/- 11)
//! test benches::bench_new_random   ... bench:       4,823 ns/iter (+/- 889)
//! test benches::bench_run          ... bench:         452 ns/iter (+/- 400)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out
//! ```
//!
//! # Conclusion
//! As a wise men once said: ["Ceterum censeo Javam delendam esse."](https://en.wikipedia.org/wiki/Carthago_delenda_est)
//!
//! I hope the Java programmers can take the joke and not hack me up into bits; also, would you stop passing around so many references?
//! `cargo` was getting hysterical...
//!
//! I may have been a bit mean to Java so feel free to [`bash`](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) on Rust. (Not that you're going to find anything to bash on. 😁)

pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod sbip;
pub mod softbody;
pub mod tile;

pub use board::*;
pub use brain::*;
pub use climate::Climate;
pub use sbip::*;
pub use softbody::*;
pub use tile::Tile;

pub trait Drawable {
    fn draw(&self);
}

pub enum Dragging {
    Board,
    MinTemperature,
    MaxTemperature,
}
