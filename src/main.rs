#![no_std]
#![no_main]

use core::str::FromStr;
use libc;
use libc_print::std_name::{println, eprintln, dbg};
// use heapless::String;

#[derive(Debug)]
enum Error {
    StringAllocationFailure,
}

// fn main() -> Result<(), Error> {
// fn main() {
//     match _main() {
//         // Ok(()) => Ok(()),
//         // Err(e) => Err(e),
//         Ok(()) => (),
//         Err(e) => {
//             dbg!(e);
//         }
//     };
// }

// fn _main() -> Result<(), Error> {
//     let s = String::<20>::from_str("Hello, world!\r\n").unwrap();
//     dbg!(s);
//     println!("Hello, stdout!");
//     eprintln!("Hello, stderr!");

//     // This will error
//     let _s = String::<5>::from_str("Hello, world!\r\n")
//         .map_err(|_| Error::StringAllocationFailure)?;

//     Ok(())
// }

fn _main() -> Result<(), Error> {
    Ok(())
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Specify the entry point for the program
#[no_mangle]
pub extern "C" fn __libc_start_main() -> ! {
    let pid = unsafe { libc::getpid() };
    // Use the libc function to print the PID or perform other actions
    // dbg!(pid);
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let pid = unsafe { libc::getpid() };
    // Use the libc function to print the PID or perform other actions
    // dbg!(pid);
    loop {}
}

// Link to the C standard library
#[link(name = "c")]
extern "C" {
    fn getpid() -> libc::pid_t;
    fn write(fd: libc::c_int, buf: *const libc::c_void, count: libc::size_t) -> libc::ssize_t;
}
