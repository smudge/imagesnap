# imagesnap

> A CLI for capturing images on macOS 📷 📸 🖼️

This crate also doubles as a Rust library. 🦀

## Installing

### via Homebrew

```
brew install smudge/smudge/imagesnap
```

### via Cargo

[Set up Rust/Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
and install from crates.io by running:

```
cargo install imagesnap
```

## Usage

### Command-Line Interface

Run the command without any arguments to output `snapshot.jpg`, captured from the default camera:

```bash
$ imagesnap
Capturing image from device "iSight"..................snapshot.jpg
```

The filename can be changed by specifying an argument. The file extension will determine the image format:

```bash
$ imagesnap shot1.tif
Capturing image from device "iSight"..................shot1.tif
```

Use the `-l` flag to list all available image capture devices:

```bash
$ imagesnap -l
iSight
DV
```

Use the `-d` flag to use a specific device:

```bash
$ imagesnap -d DV
Capturing image from device "DV"..................snapshot.jpg
```

### Rust API

In addition to a CLI, `imagesnap` can be pulled-in as a dependency for other Rust crates:

```
imagesnap = "0.0.1"
```

## Todo:

- [X] Basic functionality working (snap image to file)
- [X] Add additional opts like 'quiet' and 'warmup'
- [X] Get device selection working
- [ ] Clean up code, work on generic lib interface
- [ ] Update README with library usage
- [ ] Support additional file types? (png, tif, etc?)
- [ ] Add Linux support (via `rscam`)
- [ ] Add Windows support (via `escapi`)
- [ ] Get STDOUT and pipe detection working (macOS/linux)

## Thanks To:

* Robert Harder for the original [imagesnap](https://github.com/rharder/imagesnap) CLI
* The maintainers of the Rust [objc crate](https://github.com/SSheldon/rust-objc)
* Carol Nichols and Steve Klabnik for the [official book](https://doc.rust-lang.org/book/) on Rust

## Contributing

* Check the issue tracker and consider creating a new issue.
* Fork the project and create a new branch for your contribution.
* Write, commit, and push your contribution to your branch.
* Make sure the project builds (`cargo build`) and functionality still works as expected.
* Submit a pull request.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
