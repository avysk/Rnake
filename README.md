# Rnake

This is a (minimal) snake game in Rust, just for fun and as as an exercise.

## Build

To build you need `cargo`. Do the following:

1. `cargo install cargo-vcpkg` -- once in a lifetime.
2. Make sure that the environment variable `VCPKG_ROOT` points to the writable location and run `cargo vcpkg build`. As long as the vcpkg root survives, there is no need to repeat this step. If `VCPKG_ROOT` is not set, `target/vcpkg` will be used (recommended) and after `cargo clean` this step has to be repeated.
3. `cargo build --release`, and the final (hopefully self-contained on all platforms) executable will be generated in `target/release` directory. This step should be repeated if Rust source code changes.

## Controls

Right and left arrow keys turn the snake, well, right and left. Actually, I have made a mistake first, and the right arrow turned the snake left, and the left arrow turned the snake right. I have decided it is too much fun.

## End of game

If the snake leaves the window or hits itself, the program exits.

## Score

None.

## Future Plans

- Tests.
- More types of things appearing in the field.
- Replacement of hardcoded values with something configurable.
- Better graphics.
- Windows installer.
- Customizable controls.
- ...

## Bugs

- In-game score position is hardcoded.
- No check if start/stop messages fit the screen.
