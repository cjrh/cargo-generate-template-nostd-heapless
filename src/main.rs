#![no_std]
#![no_main]

mod utils;
mod errors;

use core::str::FromStr;
use libc_print::std_name::{println, eprintln, dbg};
use errors::Error;
use heapless::String;

fn printf<T: AsRef<str>>(s: T) {
    unsafe {
        libc::printf(s.as_ref().as_ptr() as *const _);
    }
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    match _main() {
        Ok(()) => 0,
        Err(e) => {
            println!("{e}");
            1
        }
    }
}

fn _main() -> Result<(), Error> {
    // Roll-your-own wrapper of libc printf
    printf("Hello, printf!\n");

    // Or you can just use libc_print
    dbg!("Hello, dbg!");
    println!("Hello, stdout!");
    eprintln!("Hello, stderr!");

    // Can get heap-like strings from Heapless without dynamic allocation
    let s = String::<20>::from_str("Hello, world!\r\n").unwrap();
    dbg!(s);

    // How about a vec?
    let v = heapless::Vec::<_, 4>::from_slice(&[1, 2, 3, 4]).unwrap();
    dbg!(v);

    // How about a hashmap?
    let mut h = heapless::FnvIndexMap::<_, _, 16>::new();

    // review some books.
    h.insert(123,    456).unwrap();
    dbg!(h);

    // Finally, some Rust error handling. This will fail because the string
    // is too long.
    let _s = String::<5>::from_str("Hello, world!\r\n")
        .map_err(|_| Error::StringAllocationFailure)?;

    Ok(())
}
