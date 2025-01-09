use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rerun-if-changed=wrapper.h");

  #[cfg(target_os = "macos")]
    // Tell the linker to use the VISA framework on macOS
    println!("cargo:rustc-link-arg=/Library/Frameworks/VISA.framework/Versions/Current/VISA");
  #[cfg(target_os = "windows")]
  {
    // TODO: Not tested on Windows
    // Link the VISA library on Windows
    println!("cargo:rustc-link-search=C:\\Program Files (x86)\\IVI Foundation\\VISA\\WinNT\\Bin");
    println!("cargo:rustc-link-lib=dylib=visa32");
  }
  #[cfg(target_os = "windows")]
  {
    // TODO: Not tested on Linux
    // Link the VISA library on Linux
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=dylib=visa");
  }

  #[cfg(target_os = "macos")]
  const NI_VISA_INCLUDE_PATH: &str = "/Library/Frameworks/VISA.framework/Headers";

  #[cfg(target_os = "linux")]
  pub const NI_VISA_INCLUDE_PATH: &str = "/usr/include/";

  #[cfg(target_os = "windows")]
  const NI_VISA_INCLUDE_PATH: &str = "C:\\Program Files (x86)\\IVI Foundation\\VISA\\WinNT\\Include";

  // Generate bindings
  let bindings = bindgen::Builder::default()
    .header("wrapper.h") // Point to visa.h
    .clang_arg(format!("-I{}", NI_VISA_INCLUDE_PATH))
    .clang_arg("-v")
    .generate()
    .expect("Unable to generate bindings");

  // Write bindings to the output directory
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("visa_bindings.rs"))
    .expect("Couldn't write bindings!");
}
