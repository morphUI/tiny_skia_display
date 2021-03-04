# tiny_skia_-_display

[![Build and test](https://github.com/morphUI/tiny_skia_display/workflows/CI/badge.svg)](https://github.com/morphUI/tiny_skia_display/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

<img alt="tiny-skia-display" src="https://codeberg.org/flovanco/assets/raw/branch/master/raqote-display.png">

Implementation of embedded-graphics https://github.com/jamwaffles/embedded-graphics DrawTarget  based on tiny-skia https://github.com/RazrFalcon/tiny-skia.

To include tiny-skia-display in your project, add this dependency
line to your `Cargo.toml` file:

```text
tiny-skia-display = { git = "https://github.com/morphUI/tiny-skia-display" }
```

### Run example

```shell
cargo run --example minimal
```

### Run web example

#### Install web toolchain

* Rust standard toolchain `rustup`, `rustc`, `cargo` ([install](https://www.rust-lang.org/tools/install))
* Rust web assembly toolchain `wasm-pack` ([install](https://rustwasm.github.io/wasm-pack/installer/))
* JavaScript package manager npm ([install](https://www.npmjs.com/get-npm))

#### Rub example

* Navigate to `examples/minimal` director
* Run `npm install`
* Run `npm run serve`

## Build and run documentation

You can build and view the latest documentation by executing the following command:

```shell
cargo doc --no-deps --open
```