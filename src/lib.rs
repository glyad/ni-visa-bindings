#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, unused_imports)]
pub mod ffi {
  include!(concat!(env!("OUT_DIR"), "/visa_bindings.rs"));
}

#[cfg(test)]
mod tests;