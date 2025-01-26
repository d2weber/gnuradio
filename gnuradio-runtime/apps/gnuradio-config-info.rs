#[cxx::bridge(namespace = "grust")]
mod ffi {
    extern "Rust" {
        fn hello() -> u32;
    }
}

fn hello() -> u32 {
    1337
}
