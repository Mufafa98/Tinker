pub const DEBUG: bool = true;

// Macro for conditional printing
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if crate::macros::DEBUG {
            println!($($arg)*);
        }
    };
}
