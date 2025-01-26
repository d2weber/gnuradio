use ffi::my_magic_number;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("gnuradio/my_function.h");
        fn my_magic_number() -> u32;
    }

    #[namespace = "grust"]
    extern "Rust" {
        fn hello() -> u32;
    }
}

fn hello() -> u32 {
    my_magic_number()
}
