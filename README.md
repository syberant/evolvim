# Evolv.io improved! [![Build Status](https://travis-ci.org/syberant/evolvim.svg?branch=master)](https://travis-ci.org/syberant/evolvim)

This project is a [Rust] port of the [Processing code] from [carykh] and contributors.
The code was 100% rewritten but includes most (all?) ideas of the [Processing code] with a lot merely being translated into [Rust].

# What improvements are there?

That it's written in [Rust] of course!

There are already (more coming up: WIP) a number of improvements over the original:
- WIP: better documentation to make digging around and playing with it easier
- Performance, [Processing] isn't known for anything resembling speed, at least not when compared to [Rust].
- Turning the graphics off, this option is very useful to achieve some real speed.
- Flexibility, don't like my graphical implementation? Just use `Board` and build your own!
- File saving and loading (Done!)
- Benchmarking, performance is taken seriously and methods for testing are included.
- Stability, [Rust] provides a lot of guarantees which add to the overall stability of the program.

The original version has since also [been converted](https://github.com/evolvio/evolv.io/) to pure Java
(No, I am not going to provide a link; Java isn't worthy of that).

# Installing
- [install Rust]
- clone the repository with `git clone https://github.com/syberant/evolvim`
- `cd evolvim` into the repository
- `cargo build --release` to compile the project
- proceed to [Usage](#usage) and start simulating some life!

## Running benchmarks
Use `cargo bench --features=bench` to run all benchmarks, please do note that you have to be using [nightly] for this.

# Usage
You can use the internal logic and make your own graphics-frontend or use mine (which is pretty crappy).

## My crappy frontend
This is in `target/release/evolvim` so to get the following help message type `target/release/evolvim --help`:
```
Evolvim - GUI launched via CLI 0.2.0
Sybrand Aarnoutse

USAGE:
    evolvim [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s, --save       Saves to the input file when done
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     The input file, start with this as board
    -o, --output <FILE>    The output file, save to this when done
```

### Controls for my crappy frontend
- click on a creature to select it
- `b` to select the biggest creature
- `o` to select the oldest creature
- `q` to deselect a creature
- `Up` to speed up time
- `Down` to slow down time

## The CLI implementation (use this for speed and to get a quick overview)
This is in `target/release/evolvim_cli` so to get the following help message type `target/release/evolvim_cli --help`:
```
Evolvim - cli 0.2.0
Sybrand Aarnoutse

USAGE:
    evolvim_cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --info       Output a summary of this world
    -s, --save       Saves to the input file when done
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>       The input file, start with this as board
    -u, --updates <YEARS>    Amount of years to simulate
    -o, --output <FILE>      The output file, save to this when done
```

# Documentation
As this project is very young it doesn't have good documentation yet, some can be found however by typing `cargo doc --no-deps --open`. Any further documentation is located in the "self-documenting" code...

# License
This project is licensed under the MIT license ([LICENSE](https://github.com/syberant/evolvim/blob/master/LICENSE) or http://opensource.org/licenses/MIT).

[Processing code]: https://github.com/evolvio/evolv.io/tree/120b3c1f11c6beade92343fc40f57d376b8a7434
[carykh]: https://www.youtube.com/user/carykh
[Rust]: https://rust-lang.org
[Processing]: https://processing.org
[install Rust]: https://www.rust-lang.org/en-US/install.html
[nightly]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
