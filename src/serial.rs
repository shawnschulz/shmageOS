use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;

// Used to print to Qemu's UART serial port interface
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // We wrap the raw pointer to the serial port in a safe interface
        // using spinlocks to allow the main thread loop to acquire the 
        // lock. Lazy_static ensures that the raw pointer and lock
        // is acquired exactly once at runtime, helping ensure mem safety
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        return Mutex::new(serial_port)
    };
}

// Eh who knows what this is doing lol
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed")
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[test_case]
fn test_fuzzed_serial() {
    // Come back and make the fuzzed test later when you can alloc mem
    serial_println!("hello world");
}
