# gpx-seeker
Will seek for a section of determined distance with speed above and closest to a target in a GPX file.

## Building and running

If you have Nix installed you can run:

```
nix run . -- --distance 1000 --speed 25.0 file.gpx
```

Otherwise, if you have Rust installed, you can do:

```
cargo run -- --distance 1000 --speed 25.0 file.gpx
```
