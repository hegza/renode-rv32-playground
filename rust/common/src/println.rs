#[doc(hidden)]
#[macro_export]
macro_rules! write_impl {
    ($uart:expr, $s:expr) => {{
        use core::fmt::Write;
        write!($uart, $s).unwrap()
    }};
    ($uart:expr, $($tt:tt)*) => {{
        use core::fmt::Write;
        write!($uart, $($tt)*).unwrap()
    }};
}

#[macro_export]
macro_rules! sprint {
    ($($tt:tt)*) => {{
        unsafe {
        $crate::SHARED.serial_port.as_mut().map(|uart| $crate::write_impl!(uart, $($tt)*))
        }
    }};
}

#[macro_export]
macro_rules! sprintln {
    () => {{
        $crate::sprint!("\n");
    }};
    // IMPORTANT use `tt` fragments instead of `expr` fragments (i.e. `$($exprs:expr),*`)
    ($($tt:tt)*) => {{
        unsafe {
        $crate::SHARED.serial_port.as_mut().map(|uart| {
            $crate::write_impl!(uart, $($tt)*);
            $crate::write_impl!(uart, "\n")
        })
    }
    }};
}
