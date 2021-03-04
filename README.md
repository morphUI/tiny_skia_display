# tiny-skia-display

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

To execute the [lvgl](https://github.com/rafaelcaricio/lvgl-rs) example change in the directory `examples/lvgl` and run:

```shell
DEP_LV_CONFIG_PATH=`pwd`/include cargo run
```

## Build and run documentation

You can build and view the latest documentation by executing the following command:

```shell
cargo doc --no-deps --open
```