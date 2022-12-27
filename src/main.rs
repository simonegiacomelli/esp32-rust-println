use std::time::Duration;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


fn main() {
    esp_idf_sys::link_patches();
    let mut x = 0;
    loop {
        x += 1;
        println!("Hello, world! {x}");
        // see https://github.com/ivmarkov/rust-esp32-std-demo/blob/main/src/main.rs
        std::thread::sleep(Duration::from_millis(500));
    }
}
