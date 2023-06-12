Basic and naive implementation of string basic pattern matching algorithms.

- [Rabin-Karp algorithm](https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm).
- [Bitap (shift-and) algorithm](https://en.wikipedia.org/wiki/Bitap_algorithm).

To run, install the [Rust toolchain](https://rustup.rs). Cargo may be used to
compile the source.

Usage,

```none
$ cargo run -q -- --help
Usage: patmat [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>
          The pattern

Options:
  -a <ALGORITHM>
          The pattern matching algorithm

          [default: shift-and]
          [possible values: rabin-karp, shift-and]

      --stats
          Whether the program should show statistics

  -i <INPUT>
          The file to perform the search.

          If not passed, will read from the stdin.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Example,

```none
$ cargo run -q -- --stats -i Cargo.toml path
80
108
146
(done in 20 μs)
```
