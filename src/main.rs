#![no_std]

use core::str::FromStr;
use libc_print::std_name::{println, eprintln, dbg};
use heapless::String;

#[derive(Debug)]
enum Error {
    StringAllocationFailure,
}

// fn main() -> Result<(), Error> {
fn main() {
    match _main() {
        // Ok(()) => Ok(()),
        // Err(e) => Err(e),
        Ok(()) => (),
        Err(e) => {
            dbg!(e);
        }
    };
}

fn _main() -> Result<(), Error> {
    let s = String::<20>::from_str("Hello, world!\r\n").unwrap();
    dbg!(s);
    println!("Hello, stdout!");
    eprintln!("Hello, stderr!");

    // This will error
    let _s = String::<5>::from_str("Hello, world!\r\n")
        .map_err(|_| Error::StringAllocationFailure)?;

    Ok(())
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
