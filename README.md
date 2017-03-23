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

## Contributing

Send a PR! We don't bite ;)

## License

CustomerLocator is licensed under either of the following, at your option:

 * MIT License ([COPYRIGHT](COPYRIGHT) or http://opensource.org/licenses/MIT)
