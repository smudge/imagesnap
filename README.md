# imagesnap

> A CLI for capturing images on macOS **and Linux** 📷 📸 🖼️

This crate also doubles as a Rust library. 🦀

## Installing

### via Homebrew

```
brew install smudge/smudge/imagesnap
```

### via Cargo

[Set up Rust/Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)
and install from crates.io by running:

On Linux you will also need the `libv4l2` development headers
(usually provided by the `libv4l-dev` package).

```
cargo install imagesnap
```

## Usage

### Command-Line Interface

Run the command without any arguments to output `snapshot.jpg`,
captured from the default camera:

```bash
$ imagesnap
Capturing image from device "iSight"..................snapshot.jpg
```

On Linux the default device is usually `/dev/video0` and
listed devices will appear under `/dev/video*`.

The filename can be changed by specifying an argument. Only JPG files are currently supported:

```bash
$ imagesnap shot1.jpg
Capturing image from device "iSight"..................shot1.jpg
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

Use the `-w` flag to specify a warmup period (default is 0.5), allowing the camera to perform light balancing and/or focus before taking a shot:

```bash
$ imagesnap -w 2.5
Capturing image from device "iSight"...........................snapshot.jpg
```

Use the `-q` flag to silence the status and progress output.

Lastly, run the command with `-h`/`--help` to see usage instructions.

### Rust API

In addition to a CLI, `imagesnap` can be pulled-in as a dependency for other Rust crates:

```
imagesnap = "0.0.1"
```

To snap an image with the default camera, use `Camera::default`:

```rust
let camera = Camera::default();
camera.snap("snapshot.jpg").await;
```

Note that `snap` is an `async` function.

If more than one camera is attached, use `Camera::new` and specify a device:

```rust
let camera = Camera::new(Device::find("FaceTime"), None);
```

To discover all devices, use `Device::all()`.

An optional warmup period may also be specificied (in seconds):

```
let camera = Camera::new(Device::default(), 1.5);
```

If left unspecified, it will default to 0.5 seconds.

### Bonus: a git `post-commit` hook

To automatically take a selfie with each git commit, create the following file at
`~/.git-templates/hooks/post-commit` or (in a given repo) `.git/hooks/post-commit`:

```bash
#!/bin/bash
FILE="${HOME}/.gitshots/$(date +%s)"
if [ ! -d "$(git rev-parse --git-dir)/rebase-merge" ]; then
  git log HEAD~1..HEAD --pretty=format:'{"sha1": "%H", "author": "%aN", "authored": %at, "commiter": "%cN", "committed": %ct, "msg": "%s", "project:" "' > "$FILE.json"
  git rev-parse --show-toplevel | awk -F/ '{print $NF}' | tr -d '\n' >> "$FILE.json"
  echo -n '"}' >> "$FILE.json"

  imagesnap -q -w 3 "$FILE.jpg" &
fi
exit 0
```

You may choose to change or remove `-w 3` (the warmup delay), depending on your default camera.

## Todo:

- [X] Basic functionality working (snap image to file)
- [X] Add additional opts like 'quiet' and 'warmup'
- [X] Get device selection working
- [X] Clean up code, work on generic lib interface
- [X] Update README with library usage
- [ ] Support additional file types? (png, tif, etc?)
- [X] Add Linux support (via `rscam`)
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
