#![no_std]
#![no_main]
extern crate alloc;
extern crate libc_alloc;

use libc::{c_char, c_int, exit, puts};

use serde_json_canonicalizer::pipe;

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
  let input = r#"{"b": false, "c": 12e1, "a": "Hello!"}"#;
  let expected = r#"{"a":"Hello!","b":false,"c":120}"#;
  let jcs = pipe(input).unwrap();

  if jcs.as_str() != expected {
    unsafe {
      puts(b"Canonicalization failed!\0".as_ptr().cast());
      exit(1);
    }
  }

	unsafe {
        puts(b"Tests passed!\0".as_ptr().cast());
    }
	0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  unsafe { exit(1); }
}

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}
