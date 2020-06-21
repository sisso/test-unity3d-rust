#[macro_export]
macro_rules! debug {
    () => (debug!(""));
    ($fmt:expr) => (match ::std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/ffi.log") {
            Ok(mut file) => {
                use std::io::Write;
                file.write_all(format!("{}\n", $fmt).as_bytes()).ok();
            }
            Err(_) => {
                panic!("failed to open log file")
            },
        });
    ($fmt:expr, $($arg:tt)*) => (match ::std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/ffi.log") {
            Ok(mut file) => {
                use std::io::Write;
                file.write_all(format!(concat!($fmt, "\n"), $($arg)*).as_bytes()).ok();
            }
            Err(_) => {
                panic!("failed to open log file")
            },
        });
}
