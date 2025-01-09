# ni-visa-bindings

`ni-visa-bindings` provides raw Rust bindings to the National Instruments Virtual Instrument Software Architecture (NI-VISA) library. This crate enables low-level access to the VISA API for instrument control and communication, making it a foundational component for building higher-level abstractions in Rust.

---

## Features

- **Raw Bindings**: Direct, low-level access to the NI-VISA C API.
- **Compatibility**: Works with National Instruments' implementation of VISA.
- **Foundation**: A base crate for building more idiomatic Rust wrappers or applications interacting with VISA-compliant instruments.

---

## Installation

There are two ways to add the crate to your project dependencies:

- The first way is to add the following to your `Cargo.toml`:
```toml
[dependencies]
ni-visa-bindings = "1.0.0"
```
- The second way is to execute the following CLI command in your OS shell terminal:
```bash
cargo add ni-visa-bindings
```

You will also need to have the NI-VISA library installed on your system.

- Download it from [NI-VISA Downloads](https://www.ni.com/en-us/support/downloads/drivers/download.ni-visa.html).
- Ensure the library is accessible in your system's library path.

---

## Usage

This crate exposes the raw bindings to the NI-VISA library. Below is an example of how to initialize a VISA session using these bindings:

```rust
use ni_visa_bindings::*;

fn main() {
    unsafe {
        let mut default_rm: ViSession = 0;
        let status = viOpenDefaultRM(&mut default_rm);

        if status < VI_SUCCESS {
            eprintln!("Failed to open default resource manager. Status: {}", status);
        } else {
            println!("Default resource manager opened successfully.");
            // Perform additional VISA operations here...

            // Close the resource manager
            viClose(default_rm);
        }
    }
}
```

Additionally, you can refer to the [tests provided in the source code](https://docs.rs/crate/ni-visa-bindings/1.0.0/source/src/ffi/tests/mod.rs) as an excellent knowledge source. These tests demonstrate various usages of the bindings and serve as practical examples to guide development.

---

## Prerequisites

- **Rust**: Ensure you have the Rust toolchain installed. You can get it from [rustup](https://rustup.rs/).
- **NI-VISA**: Install the NI-VISA library from National Instruments.

---

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/ni-visa-bindings).

---

## Contributing

Contributions are welcome! If you encounter a bug, have a feature request, or want to improve the code, feel free to open an issue or submit a pull request.

### Development Setup

1. Clone the repository:
   ```sh
   git clone https://github.com/<your-username>/ni-visa-bindings.git
   cd ni-visa-bindings
   ```
2. Build the project:
   ```sh
   cargo build
   ```
3. Run tests:
   ```sh
   cargo test
   ```

---

## Roadmap
       
      |                                  | Implemented in    | Implemented in  |
      | Feature                          | Original Library  | crate           |
      |----------------------------------|-------------------|-----------------|
      | Provide the same functionality,  |                   |                 |
      | as in original library           |      ✅           |        ✅        |
      | Documentation                    |      ✅           |        ⬜        |
      | Expanded test coverage           |      ⬜           |        ⬜        |
      | Add more comprehancive examples  |      ⬜           |        ⬜        |

Contributions and suggestions for the roadmap are welcome! Feel free to open an issue or discussion.

---

## License

This project is licensed under the [Unlicense License](https://github.com/glyad/ni-visa-bindings/blob/main/LICENSE).

---

## Acknowledgments

- National Instruments for providing the NI-VISA library and documentation.
- The Rust community for supporting FFI and low-level library development.

---

## Disclaimer

This crate is not officially affiliated with or endorsed by National Instruments. It is an independent open-source project.
