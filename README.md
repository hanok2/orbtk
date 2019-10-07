<img alt="OrbTk" width="380" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/logos/orbtk/logo_dark.png">

[![Build status](https://gitlab.redox-os.org/redox-os/orbtk/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.27-orange.svg)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

> OrbTk 0.3.0 is under heavy development and is not compatible with earlier releases.

The Orbital Widget Toolkit is a multi platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity_component_system) and provides a functional-reactive like API.

The main goals of OrbTk are speed, ease of use, and being cross platform.

<img alt="Calculator" width="350" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern lightweight API
* Uses the Entity Component System library [DCES](https://gitlab.redox-os.org/redox-os/dces-rust) for widget and properties handling
* Updating instead of rebuilding sub-trees
* Flexible event system
* Widget state management
* Cross platform: Redox OS, Linux, macOS, Windows
* CSS theming

## Usage

To include OrbTk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.2.27"
```

To use OrbTk 0.3, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = { git = "https://gitlab.redox-os.org/redox-os/orbtk.git" }
```

## Minimal Example

```rust
use orbtk::prelude::*;

fn main() {
      Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
```

## Additional Examples

You can find examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

## Additional Examples on Web

To run the examples on a browser you have to install

```text
cargo install -f cargo-web
```

### Run

You can start the widgets example by executing the following command:

* Compile to [WebAssembly](https://en.wikipedia.org/wiki/WebAssembly) using Rust's native WebAssembly backend:

```text
cargo web start --target=wasm32-unknown-unknown --auto-reload --example widgets
```

* Compile to [asm.js](https://en.wikipedia.org/wiki/Asm.js) using Emscripten:

```text
cargo web start --target=asmjs-unknown-emscripten --auto-reload --example widgets
```

* Compile to WebAssembly using Emscripten:

```text
cargo web start --target=wasm32-unknown-emscripten --auto-reload --example widgets
```

## Build and run documentation

You can build and run the latest documentation by executing the following command:

```text
cargo doc --no-deps --open
```

## Planned features

* Style guide
* More default widgets
* More examples
* Book
* Animations
* Exchange views / widgets / screens on runtime
* Split application in modules
* Theme update
* Support for Android, iOS, Ubuntu Touch and WebAssembly
* 3D support

## Sub Crates

* [api](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/api): base api elements of OrbTk e.g. widget and application parts
* [css-engine](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/css-engine): parse and read values from a css file
* [render](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/render): cross platform 2D/3D render library
* [shell](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/api): cross platform window and event handling
* [theme](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/theme): OrbTks default theme (light and dark)
* [tree](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/tree): tree structure based on DCES
* [utils](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/utils): helper structs and traits
* [widgets](https://gitlab.redox-os.org/redox-os/orbtk/tree/master/crates/widgets): base widget library

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

## License

Licensed under MIT license ([LICENSE](.LICENSE)).