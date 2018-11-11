# Evolv.io improved! [![Build Status](https://travis-ci.org/syberant/evolvim.svg?branch=master)](https://travis-ci.org/syberant/evolvim)

This project is a [Rust] port of the [Processing code] from [carykh] and contributors.
The code was 100% rewritten but includes most (all?) ideas of the [Processing code] with a lot merely being translated into [Rust].

# Why is this improved?

Because it's in [Rust] of course!

There are (or will be: WIP) a number of reasons why this version is (or will be) an improvement over the original:
- WIP: better documentation
- Performance, [Processing] isn't known for anything resembling speed, at least not when compared to [Rust].
- Flexibility, don't like my graphical implementation? Just use `Board` and build your own!
- File saving and loading (Done!)

The original version has since also [been converted](https://github.com/evolvio/evolv.io/) to pure Java
(No, I am not going to provide a link; Java isn't worthy of that).
This version still provides:
- a (subjectively) better language (if you haven't noticed: I hate Java)
- WIP: (hopefully) less bugs because of [Rust]'s guarantees
- nice benchmarking tools
- an option to turn the graphics off to enhance performance
- maybe more but I AM NOT LOOKING AT JAVA CODE ANY LONGER TO COMPARE

# Installing
- [install Rust]
- clone the repository with `git clone https://github.com/syberant/evolvim`
- run my graphical implementation with `cargo run --release --bin evolvim` (`--release` optimizes the code and `--bin evolvim` specifies what to run)

## Running benchmarks
Use `cargo bench --features=bench` to run all benchmarks.

# Usage
You can use the internal logic and make your own graphics-frontend or use mine (which is pretty crappy).

## Controls for example frontend
- click on a creature to select it
- `b` to select the biggest creature
- `o` to select the oldest creature
- `q` to deselect a creature
- `Up` to speed up time
- `Down` to slow down time


# Documentation
As this project is very young it doesn't have good documentation yet, some can be found however by typing `cargo doc --no-deps --open`.

# License
This project is licensed under the MIT license ([LICENSE](https://github.com/syberant/evolvim/blob/master/LICENSE) or http://opensource.org/licenses/MIT).

[Processing code]: https://github.com/evolvio/evolv.io/tree/120b3c1f11c6beade92343fc40f57d376b8a7434
[carykh]: https://www.youtube.com/user/carykh
[Rust]: https://rust-lang.org
[Processing]: https://processing.org
[install Rust]: https://www.rust-lang.org/en-US/install.html
