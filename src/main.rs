#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

// this function is the entry point, since the linker looks for a function
// named `_start` by default
// extern "C" means use the C calling convention instead of Rusts
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
  println!("Hello World!");

  loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}