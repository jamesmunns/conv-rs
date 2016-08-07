# Basic Convolution

This is a toy library to demonstrate some linkage, and performance differences between rust and other languages.

Don't really use this.

# Benchmarking

To run a benchmark:
``` bash
    $ rustup default nightly
    $ cargo bench --features unstable

        running 2 tests
        test tests::basic_test ... ignored
        test bench::bench_test ... bench:  14,624,499 ns/iter (+/- 3,028,571)

        test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured
```

