# Customer Locator

CustomerLocator is a small CLI app that allows you to load customer data from
a JSON file and be a able to do search on proximity based on a radius given
in Kilometers.

## Design

If you are interested on the design considerations take a look at [DESIGN](DESIGN.md).

## Building and Running

### Installing Rust

CustomerLocator is built in Rust so you will need to install it. To install Rust,
run the following in your terminal, then follow the onscreen instructions:

```sh
$ curl https://sh.rustup.rs -sSf | sh
```

That will install `rustup` the Rust version manager. You can install the latest
stable Rust release with:

```sh
$ rustup install stable
````

Once you have installed Rust and fetched the source code you can build and run it
using `cargo`:

```sh
$ cargo run
```

You can pass options to the application with `--` like:

```sh
$ cargo run -- -r 50 # change the default radius to 50 km
```

## Tests

You can run the test suite with:

```sh
$ cargo test
```

## Performance

Simple tests show that CustomerLocator can parse and locate 1 million customers in less than 2 seconds.
with most of the time spent on IO pushing results to stdout.

You can run it on your own the 'customers_huge.json' file is provided compressed with the distribution.

NOTE: The --release flag is key as it will enable compiler optimizations.

```sh
$ time cargo run --release -- -f data/customers_huge.json
```

```sh
$ cargo run --release -- -f data/customers_huge.json  1.80s user 0.82s system 56% cpu 4.610 total
```

When we don't print results to stdout things get a lot faster:


```bash
$ time cargo run --release -- -f data/customers_huge.json -q # note the -q for 'quiet' mode so we don't go to stdout
```

Now we are in subsecond times \o/:

```bash
$ cargo run --release -- -f data/customers_huge.json -q  0.86s user 0.15s system 84% cpu 1.195 total
```

## Contributing

Send a PR! We don't bite ;)

## License

CustomerLocator is licensed under either of the following, at your option:

 * MIT License ([COPYRIGHT](COPYRIGHT) or http://opensource.org/liacenses/MIT)
