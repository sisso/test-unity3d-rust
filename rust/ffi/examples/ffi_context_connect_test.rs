use ffi_domain::ffi::FfiContext;
use std::time::Duration;

fn main() {
    let mut ffi_context = FfiContext::new(Some("localhost:3333"));

    loop {
        match ffi_context.take() {
            Ok(Some((kind, bytes))) => {
                let bytes_b64 = base64::encode(bytes);
                println!("receive {:?}: {:?}", kind, bytes_b64);
            }

            Ok(None) => {
                println!("receive none");
            }

            Err(e) => {
                eprintln!("receive error {:?}", e);
            }
        }

        std::thread::sleep(Duration::from_millis(90));
    }
}
