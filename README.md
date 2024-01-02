# Mentors

# Mentors

"Mentors" is a robust and user-friendly tool designed to edit and manage JSONL text datasets efficiently. Tailored for data professionals, researchers, and developers, it simplifies the process of modifying text data with features like adding, deleting, and altering text entries.

![Imgur Image1](https://i.imgur.com/AH4aq4l.png)

## Prerequisites

Before you begin, ensure you have met the following requirements:
- Rust (edition 2021)
- Trunk

You can install Rust and Trunk:
- Rust: Follow instructions at [rust-lang.org](https://www.rust-lang.org/tools/install)
- Trunk: Follow instructions at [Trunk](https://trunkrs.dev/)

## Building the Application

Build the application using Cargo: 

```bash
cargo build
```

### Running

```bash
trunk serve
```

Allows you to go to the application and use it on your computer.

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

This will start a local server and the application will be accessible in your web browser.

## Features and Dependencies

- Yew: A modern Rust framework for creating multi-threaded front-end web apps using WebAssembly.
- Serde: A framework for serializing and deserializing Rust data structures.
- wasm-bindgen and wasm-bindgen-futures: Facilitate high-level interactions between Wasm modules and JavaScript.
- js-sys: Provides bindings to global JavaScript functions and objects.

## License

Distributed under the MIT OR Apache-2.0 License. See `LICENSE` for more information.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## Contact

For support or inquiries, please open an issue on the [GitHub repository](https://github.com/teilomillet/mentors/issues).

## Acknowledgements

- [Yew](https://yew.rs/)
- [Trunk](https://trunkrs.dev/)
