# ni-visa-bindings

`ni-visa-bindings` provides raw Rust bindings to the National Instruments Virtual Instrument Software Architecture (NI-VISA) library. This crate enables low-level access to the VISA API for instrument control and communication, making it a foundational component for building higher-level abstractions in Rust.

---

## Features

- **Raw Bindings**: Direct, low-level access to the NI-VISA C API.
- **Compatibility**: Works with National Instruments' implementation of VISA.
- **Foundation**: A base crate for building more idiomatic Rust wrappers or applications interacting with VISA-compliant instruments.

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ni-visa-bindings = "0.1"
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

## License

This project is licensed under the [Unlicense License](https://github.com/glyad/ni-visa-bindings/blob/main/LICENSE).

---

## Acknowledgments

- National Instruments for providing the NI-VISA library and documentation.
- The Rust community for supporting FFI and low-level library development.

---

## Disclaimer

This crate is not officially affiliated with or endorsed by National Instruments. It is an independent open-source project.
