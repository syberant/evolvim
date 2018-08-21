//! Evolv.io improved!
//!
//! This project is a [Rust] port of the original [Processing code](https://github.com/evolvio/evolv.io/tree/120b3c1f11c6beade92343fc40f57d376b8a7434) from [carykh](https://www.youtube.com/user/carykh).
//!
//! # Why is this improved?
//!
//! Because it's in [Rust] of course!
//!
//! There are (or will be: WIP) a number of reasons why this version is better than the original:
//! - WIP: better documentation
//! - Performance, [Processing] isn't known for anything resembling speed, at least not when compared to [Rust].
//!
//! The original version has since also [been converted](https://github.com/evolvio/evolv.io/) to Java.
//! (No, I am not going to provide a link; Java isn't worthy of that.)
//! This version still provides:
//! - a (subjectively) better language (if you haven't noticed: I hate Java)
//! - (very probably) better performance plus the option to turn off the graphics
//! - (hopefully) less weird bugs because of [Rust]'s capability of safe memory management
//! - maybe more but I AM NOT LOOKING AT JAVA CODE TO COMPARE ANY LONGER
//!
//! # Performance
//!
//! ## Graphics
//! At the time of writing, my implementation is ABSOLUTELY TERRIBLE.
//! Please do use the option to turn the graphics off.
//! That will show the speed of the internal logic of `evolvim`, which is pretty good (and can improve even more with multithreading).
//!
//! ## Running benchmarks
//! I have written some benchmarks to assess the performance of this crate.
//! You can run them with `cargo bench --features=bench`.
//! Please note that this only works in nightly (`rustup default nightly` to use it by default).
//! Type `rustup toolchain install nightly` to install the latest nightly build; `rustup toolchain uninstall nightly` to uninstall again.
//!
//! ## Sudden performance boosts or dips
//! So, you played around a bit and it's now twice, or even 100 times, as fast?
//! This is most likely caused by a change to the constants in `constants.rs`,
//! the benchmarks use `DEFAULT_BOARD_SIZE` for example to test the performance of a `Board` with that size!
//! This can cause huge performance improvements or, well, a huge decline in performance...
//! If you're sure you didn't change the scale of calculations but only made them more efficient: please submit a pull request on github!
//! Performance is a critical part of this application and any improvements are welcome!
//!
//! ## Multithreading
//! This is on my list but could be a bit tricky because of my usage of `unsafe` (for information about `safe` and `unsafe` [Rust], see [here](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html)).
//! I'm working on removing any use of `unsafe` which will significantly ease the process of converting to multithreaded code.
//!
//! # Conclusion
//! As a wise men once said: ["Ceterum censeo Javam delendam esse."](https://en.wikipedia.org/wiki/Carthago_delenda_est)
//!
//! I hope the Java programmers can take the joke and not hack me up into bits; also, would you stop passing around so many references inside of your classes?
//! `cargo` was getting hysterical...
//!
//! I may have been a bit mean to Java so feel free to [bash] on Rust.
//! (Not that you're going to find anything to [bash] on. 😁)
//!
//! For anybody interested in learning [Rust], [this](https://doc.rust-lang.org/book/2018-edition/index.html) is a great place to start.
//!
//! ## Feeling intimidated by Rust?
//! If you're already a programmer I suggest the following route (maybe it was my route):
//!
//! - try [C](https://en.wikipedia.org/wiki/C_%28programming_language%29) or [C++](https://isocpp.org) or another low-level programming language and get frustrated with the many bugs, crashes and segfaults...
//! - come to [Rust]! Start reading your way through [the book](https://doc.rust-lang.org/book/2018-edition/index.html) and get scared off...
//! - have a final go with your low-level language of choice and decide that this time, you will get the luxury of proper memory management...
//! - learn [Rust].
//!
//! For those who are new to programming: try something like [Python] or [Processing] first,
//! they're both newbie-friendly programming languages with a large and active community.
//!
//! Also, you don't have to know [Rust] to tinker with the constants in `constants.rs`;
//! they can change the way `evolvim` behaves a lot. Feel free to experiment with them!
//!
//! [Rust]: https://rust-lang.org
//! [bash]: https://en.wikipedia.org/wiki/Bash_(Unix_shell)
//! [Processing]: https://processing.org
//! [Python]: https://python.org

pub mod board;
pub mod brain;
pub mod climate;
pub mod constants;
pub mod graphics;
pub mod sbip;
pub mod softbody;
pub mod terrain;

pub use board::*;
pub use brain::*;
pub use climate::Climate;
pub use sbip::*;
pub use softbody::*;
pub use terrain::*;
